# 振动传感器

## 核心器件

STM32L432KCU6 MCU
ADXL345 三轴加速度传感器


## 原理

ADXL345 测得加速度 -> SPI 传输到 STM32L432KCU6 -> 处理数据 -> 4mA-20mA(loop)标准工业电流输出
[video](https://www.bilibili.com/video/BV1QK4y1o7dT/?spm_id_from=333.788.videopod.sections&vd_source=188a5e02d520f745e2a0cd650b30aa4b)

## 硬件架构

24V输入,经过整流桥,再经过 LDO -> 5V->LDO ->3.3V -> MCU ADXL345
5V 供电给运放,
LDO静态电流要小,
MCU运行电流要小, 使用 Lov power run mode, MSI 2 Mb, 以减少功耗

## STM32cubeMX 设置

GPIO{
    PB4->GPIO_EXTI4,
    PB3->GPIO_EXTI3,
    PB0->GPIO_OUTPUT,LOW,pull_down,low_speed.
}
clock{
    MSI 4Mhz-> 4Mhz sysclk
}
power{
    supply 3.3V, 
    low power run mode -> Power regulator voltage scale 2
    internal power voltage detector, PVD detection Level 2.9V
}
nvic{
    PB3->1
    PB4->2
}
debug{
    SWD,
}
DAC1_OUT1{
    PA4,
    only to external pin,
    trigger: None,
    output buffer: Enable
}
SPI1{
    PA1->SPI_SCK,
    PA2->SPI_MISO,
    PA3->SPI_MOSI,
    Full-Duplex Master mode,
    date size 8bit,
    MSB first,
    baud rate 500Kb,
    CPOL=1, CPHA = 1,
    CRC->disable,
}

## 焊接调试

电源调试需要排针和短接帽做隔离
MCU 烧录口为SWD,注意需要RESET才能烧录固件.
SPI口需要引出,方便使用逻辑分析仪调试.
元器件用1206封装,方便焊接.
焊接板子时,板子需要预热,然后用锡膏焊接.

## 测试SPI通讯, readID

spi.h

```c
uint8_t ADXL345_ReadID(void);
```

spi.c

```C
uint8_t ADXL345_ReadID(void){
    uint8_t id = 0xE5; //ADXL345 ID is 0xE5
    uint8_t id_reg = 0x00; //ADXL345 ID register is 0x00
    uint8_t spi_read_flag = 0x80; //SPI read flag is 0x80
    uint8_t tx_buf = spi_read_flag | id_reg; 
    uint8_t rx_buf = 0;

    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_RESET); // choose ADXL345
    HAL_SPI_TransmitReceive(&hspi1, &tx_buf, &rx_buf, 2, HAL_MAX_DELAY);
    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET); 

    if (rx_buf == id){
        return 1;
    }else{
        return 0;
    }
}
```

main.c
```
#include "stdio.h"
#include "string.h"
#include  "stdlib.h"
#include "math.h"

uint8_t id = ADXL345_ReadID();
if (id == 1){
    printf("ADXL345 ID read success!\n");
}else{
    printf("ADXL345 ID read failed!\n");
}
```

通过逻辑分析仪查看SPI通讯,最好逻辑分析仪采样率 / 10 > SPI通讯频率.

## 官方代码修改

ADXL345有官方的代码,[driver](https://github.com/analogdevicesinc/no-OS/tree/main/drivers/accel/adxl345)
只需要这两个文件,把多余的代码删除.

引入代码

```C
#inlcude "spi.h"
extern SPI_HandleTypeDef hspi1;
uint8_t read_reg(uint8_t reg){
    uint8_t tx_buf = 0x80 | reg; //SPI read flag is 0x80
    uint8_t rx_buf = 0;
    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_RESET); // choose ADXL345
    // 通过debug 发现,第二次读取第一次的寄存器数据,所以需要读两次.
    HAL_SPI_TransmitReceive(&hspi1, &tx_buf, &rx_buf, 2, HAL_MAX_DELAY);
    HAL_SPI_TransmitReceive(&hspi1, &tx_buf, &rx_buf, 2, HAL_MAX_DELAY);
    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET); 
    return rx_buf;
}
void write_reg(uint8_t reg, uint8_t data){
    uint8_t tx_buf[2] = {reg, data};
    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_RESET); // choose ADXL345
    HAL_SPI_Transmit(&hspi1, tx_buf, 2, HAL_MAX_DELAY);
    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET);
}
void read_regs(uint8_t reg, uint8_t *data, uint8_t len) {
    uint8_t tx_buf[32];  // Buffer large enough for any reasonable read
    uint8_t rx_buf[32];
    
    // First byte contains read flag and register address
    tx_buf[0] = 0x80 | reg;  // SPI read flag is 0x80
    
    // For multi-byte reads, we need to set the multi-byte bit (0x40)
    if (len > 1) {
        tx_buf[0] |= 0x40;  // Set MB bit for multi-byte read
    }
    
    // Fill the rest of the transmit buffer with dummy bytes
    for (uint8_t i = 1; i < len + 1; i++) {
        tx_buf[i] = 0x00;  // Dummy bytes for reading
    }
    
    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_RESET); // CS low
    HAL_SPI_TransmitReceive(&hspi1, tx_buf, rx_buf, len + 1, HAL_MAX_DELAY);
    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET); // CS high
    
    // Copy received data (skip first byte which is dummy)
    for (uint8_t i = 0; i < len; i++) {
        data[i] = rx_buf[i + 1];
    }
}
void write_regs(uint8_t reg, uint8_t *data, uint8_t len) {
    uint8_t tx_buf[32];  // Buffer large enough for any reasonable write
    
    // First byte contains write flag (0x00) and register address
    tx_buf[0] = reg & 0x3F;  // SPI write flag is 0x00 (top bits cleared)
    
    // For multi-byte writes, we need to set the multi-byte bit (0x40)
    if (len > 1) {
        tx_buf[0] |= 0x40;  // Set MB bit for multi-byte write
    }
    
    // Copy the data to transmit buffer
    for (uint8_t i = 0; i < len; i++) {
        tx_buf[i + 1] = data[i];
    }
    
    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_RESET); // CS low
    HAL_SPI_Transmit(&hspi1, tx_buf, len + 1, HAL_MAX_DELAY);
    HAL_GPIO_WritePin(GPIOB, GPIO_PIN_0, GPIO_PIN_SET); // CS high
}
```

## 驱动写好了,可以写主程序了.

根据[quick start guide](https://www.analog.com/en/resources/app-notes/an-1077.html)
初始化ADXL345,
读出XYZ轴数据,
转为g值,
转为0.3~3.0V DAC_OUT1 线性输出

## make

Driver 文件太大,而且可以用STM32CubeMX生成,所以不放在仓库里了.
这里可以用stm32l432kcu6.ioc从新生成.
通过软连接来实现配置.
ld的ram 分配

## openocd(pyocd) + gdb-multiarch + bash 远程调试,输出关心变量的值

openocd.cfg

```txt
source [find board/stm32l4discovery.cfg]
```

gdb-multiarch debug.gdb 调试查看需要变量的值.
为了避免 readid被优化
makefile文件中添加
CFLAGS += -O0 -g3
或者volatile告诉编译器不要优化readid

比如main.c中有代码如下

```c
    //volatile告诉编译器不要优化readid 在main.c的106行
    volatile uint8_t readid = ADXL345_ReadID(); 
    if (readid == driverid){
        printf("ADXL345 ID read success!\n");
    }else{
        printf("ADXL345 ID read failed!\n");
    }
```

debug.gdb 文件, 

```txt
file build/vibration_sensors.elf
target extended-remote :3333
set print asm-demangle on
set backtrace limit 32
monitor arm semihosting enable  # 先启用半主机
load                            # 再加载程序

break main.c:107                # volatile uint8_t readid = ADXL345_ReadID(); 的下一行代码处,保证readid已经被赋值
commands
  printf "[DEBUG] readid = 0x%02x\n", readid
  continue
end

monitor reset halt              # 复位并暂停
continue                        # 启动程序
```

注意,实际运用中,gdb文件中不能有中文,否则会报错,所以上面的注释要删除.

```bash
Error: too many arguments
debug.gdb:6: Error in sourced command file:
Invalid download offset:再加载程序.
```

bash 自动化命令 auto.sh

```bash
#!/bin/bash

# pkill -f "openocd"
pkill -f "pyocd"
pkill -f "gdb-multiarch"

# pyocd list --targets
# openocd -f openocd.cfg &
pipx run pyocd gdbserver --target stm32l432kc & # 后台运行
gdb-multiarch -x debug.gdb
```

命令行执行auto.sh

```bash
#赋予auto.sh可执行权限
chmod +x auto.sh
#执行auto.sh
./auto.sh
```

## 通过git github 管理代码

echo "# stm32l432kcu6" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin 
git push -u origin main 
推送到main分支,并关联

git push origin main:stm32cubemx_start 推送到stm32cubemx_start分支,但不关联
git checkout -b stm32cubemx_start 切换到stm32cubemx_start

## STM32CubeMX再次生成代码,

Core 是全部重要的代码,需要移开保存,生成后再加入.
.mxproject记录生成了makefile 或 keil 等工程文件, 这删除
其他并无影响,然后可以使用ioc来生成keil文件.

## killkeil.bat

```bat
@echo off
echo Cleaning Keil project...

REM 删除所有生成的输出文件
del /s /q *.axf
del /s /q *.hex
del /s /q *.bin
del /s /q *.map
del /s /q *.lst
del /s /q *.o
del /s /q *.d
del /s /q *.crf
del /s /q *.lnp
del /s /q *.dep
del /s /q *.iex
del /s /q *.htm
del /s /q *.sct

REM 删除所有生成的中间文件
del /s /q *.build_log.htm
del /s /q *.trail
del /s /q *.plg
del /s /q *.bak

REM 删除所有生成的调试文件
del /s /q *.dbg
del /s /q *.ini

REM 删除所有生成的临时文件
del /s /q *.tmp
del /s /q *.log

REM 删除所有生成的列表文件
del /s /q *.lst
del /s /q *.uvguix.*
del /s /q *.scvd
rd /s /q "./MDK-ARM/DebugConfig"
rd /s /q "./MDK-ARM/RTE"

echo Keil project cleaned successfully.
pause
```

## 后续优化空间

运放和三极管构成的loop精度依赖于电阻的精度
可以更改结构,使用更高精度的电流环结构.