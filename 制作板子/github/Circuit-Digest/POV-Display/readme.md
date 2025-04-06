[project](https://github.com/Circuit-Digest/POV-Display?tab=readme-ov-file)

[description](https://circuitdigest.com/microcontroller-projects/diy-pov-display-using-ESP32-Arduino)

[image](https://circuitdigest.com/calculators/pov-display-image-to-code-converter)

[74HC595](https://github.com/maketronica/MultiShiftRegister)

please the MultiShiftRegister .cpp .h file in the code

[install flatpak](https://linuxcapable.com/how-to-install-flatpak-on-debian-linux/)

[install arduino](https://flathub.org/apps/cc.arduino.IDE2)

# quick study arduino
https://docs.arduino.cc/software/ide/

# https://docs.espressif.com/projects/arduino-esp32/en/latest/installing.html

# ESP32-WROOM-32D 支持 esp32 by Espressif Systems
# 机场全局，下载依赖板包会快

# 选择依赖
Tools > Board > ESP32 Arduino > ESP32 WROOM Module

# 点击 check
# output
Sketch uses 1176362 bytes (89%) of program storage space. Maximum is 1310720 bytes.
Global variables use 22428 bytes (6%) of dynamic memory, leaving 305252 bytes for local variables. Maximum is 327680 bytes.


# 用户加入usb组解决权限问题
sudo usermod -a -G dialout $USER

# linux头
uname -r  # 查看当前内核版本（例如输出：6.1.0-32-amd64）
sudo apt update
sudo apt install linux-headers-$(uname -r)

# cp210x驱动（顺便安装）
sudo apt install -y python3-serial python3-pip
sudo pip3 install pyserial

# ch340驱动
sudo apt install -y build-essential
git clone https://github.com/juliagoda/CH341SER.git
cd CH341SER
make
sudo make load

# 检查
lsmod | grep ch34x

# 供电问题
发现电源灯有时不亮，再次检查原理图，发现usb cc1 cc2 未分别接 5.1k 下拉电阻，导致usb口事实上不供电。
我下意识认为usb该会供电，未仔细研究原理图。

# 解决供电问题 ch340驱动问题 验证 sub
lsusb

# 出现，说明已经识别了ch340
Bus 001 Device 007: ID 1a86:7522 QinHeng Electronics CH340 serial converter

# 选择端口
Tools > Port > /dev/ttyUSB0（CH340K 通常占用此端口）

# 下载程序
ESP32 需要在上传时进入 下载模式：
按住 BOOT 按钮。
按下 EN/RST 按钮并释放。
在 Arduino IDE 中点击 上传。
等待编译完成后，释放 BOOT 按钮。

# output
Sketch uses 1176362 bytes (89%) of program storage space. Maximum is 1310720 bytes.
Global variables use 22428 bytes (6%) of dynamic memory, leaving 305252 bytes for local variables. Maximum is 327680 bytes.
esptool.py v4.8.1
Serial port /dev/ttyUSB0
Connecting....
Chip is ESP32-D0WD (revision v1.1)
Features: WiFi, BT, Dual Core, 240MHz, VRef calibration in efuse, Coding Scheme None
Crystal is 40MHz
MAC: ec:c9:ff:46:e3:fc
Uploading stub...
Running stub...
Stub running...
Changing baud rate to 921600
Changed.
Configuring flash size...
Flash will be erased from 0x00001000 to 0x00007fff...
Flash will be erased from 0x00008000 to 0x00008fff...
Flash will be erased from 0x0000e000 to 0x0000ffff...
Flash will be erased from 0x00010000 to 0x0012ffff...
Compressed 24976 bytes to 15972...
Writing at 0x00001000... (100 %)
Wrote 24976 bytes (15972 compressed) at 0x00001000 in 0.5 seconds (effective 388.9 kbit/s)...
Hash of data verified.
Compressed 3072 bytes to 146...
Writing at 0x00008000... (100 %)
Wrote 3072 bytes (146 compressed) at 0x00008000 in 0.0 seconds (effective 599.4 kbit/s)...
Hash of data verified.
Compressed 8192 bytes to 47...
Writing at 0x0000e000... (100 %)
Wrote 8192 bytes (47 compressed) at 0x0000e000 in 0.1 seconds (effective 813.5 kbit/s)...
Hash of data verified.
Compressed 1176512 bytes to 238328...
Writing at 0x00010000... (6 %)
Writing at 0x00069855... (13 %)
Writing at 0x00080ba7... (20 %)
Writing at 0x0009c7ea... (26 %)
Writing at 0x000c6cba... (33 %)
Writing at 0x000e9efa... (40 %)
Writing at 0x000fabf9... (46 %)
Writing at 0x00100b70... (53 %)
Writing at 0x00106407... (60 %)
Writing at 0x0010b6bc... (66 %)
Writing at 0x00110c77... (73 %)
Writing at 0x00116460... (80 %)
Writing at 0x0011fbe3... (86 %)
Writing at 0x001265cc... (93 %)
Writing at 0x0012bf8f... (100 %)
Wrote 1176512 bytes (238328 compressed) at 0x00010000 in 9.8 seconds (effective 962.0 kbit/s)...
Hash of data verified.

Leaving...
Hard resetting via RTS pin...

# 现象
上电成功，下面的一排led灯会全亮，3.3V有供电的时候，LDO旁边的led灯会亮，usb有供电的时候，usb旁边的power灯会亮，usb有通讯的时候，usb旁边的信号灯会闪烁。
下载完成后，下面的一排led会灭，
再次上电后，下面的一排led会亮一下就灭。

# 这里有霍尔传感器

不知道是不是需要检测磁场变化来控制led亮灭进程？


