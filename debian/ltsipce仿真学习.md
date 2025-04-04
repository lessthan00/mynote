## ltSpice

[TI](http://www.ti.com/design-resources/design-tools-simulation/models-simulators/spice-library.html?keyMatch=SPICE&tisearch=Search-EN-everything)
[ST](https://www.st.com/content/st_com/en/search.html#q=spice-t=resources-page=1)
[Intusoft](http://www.intusoft.com/models.htm)
[Earthlink](http://www.home.earthlink.net/~wksands/model_f.htm)
[Onsemi](https://www.onsemi.cn/support/design-resources/models)
[NXP](https://www.nxp.com.cn/search?keyword=spice%20models)
[spicemodel](https://www.spicemodel.com/)
[infineon](https://www.infineon.com/cms/en/design-support/finder-selection-tools/product-finder/simulation-model/)
[microchip](https://www.microchip.com/doclisting/TechDoc.aspx?type=Spice)
[diodes](https://www.diodes.com/zh/design/tools/spice-models/)



### ltspice commend

```ltspice
.tran 10m
原始正弦波
SINE(0 1 1K)
相位90°
SINE(0 1 1K 0 0 90)
延迟1ms
SINE(0 1 1K 1m)
延迟1ms,输出两个正弦波
偏置电压0V,波幅1V,频率1K,延迟1ms,衰减0,相位偏移0°,输出波2个
SINE(0 1 1K 1m 0 0 2)
延迟1ms,输出两个正弦波
SINE(0 1 1K 0 1000 0 0)
```

```ltspice
.tran 50m
方波
PULSE(0 1 0 1n 1n 1m 2m)
方波,延迟2m
PULSE(0 1 2m 1n 1n 1m 2m)
梯台波
低电平0V,高电平1V,0s上升,tr(0%-100%)=0.5ms, 维持时间0.5ms,下降tr=0.5ms,周期时间2ms
PULSE(0 1 0 0.5m 0.5m 0.5m 2m)
三角波
PULSE(0 1 0 0.5m 0.5m 0 1m)
锯齿波
PULSE(0 1 0 1m 1n 0 1m)
```

```ltspice
.tran 50m
低电平0V,高电平1V,5ms上升 tr(10%-90%)=1ms, 30ms下降,tr=1ms
EXP(0 1 5m 1m 30m 1m)
EXP(1 0 5m 2m 30m 1m)
EXP(0 1 0 2m 20m 5m)
EXP(0 1 3m 1m 20m 1m)
```

```ltspice
.tran 50m
sig 信号波
SINE(0 1 100)
carrer 载波
SINE(0 1 1K)
sffm 合成波
偏置电平0V,波幅1V,频率1Khz,码元5,信号频率100
SFFM(0 1 1K 5 100)
```

```ltspice
.tran 32
编辑折线
PWL(0 0 2 5 3 1 7 0 8 3)
repeat3
PWL repeat for 3 (0 0 2 5 3 1 7 0 8 3) endrepeat
从文件读取
PWL file=wave.txt
C语言参数的文件
PWL file=gan.txt
repeat2
PWL repeat for 3 file=wave.txt endrepeat
```

gan.txt

```txt
0.000000000000000e+00 0.000000e+00
4.793623980846196e-06 3.015694e-02
9.587247961692393e-06 6.022677e-02
1.438087194253859e-05 9.020947e-02
1.917449592338479e-05 1.201050e-01
2.396811990423098e-05 1.499135e-01
2.876174388507718e-05 1.796348e-01
3.355536786592336e-05 2.092691e-01
6.710979973184647e-05 4.092512e-01
1.342186634636927e-04 7.467552e-01
```

wave.txt

```txt
1n 0
1 3
2 0
4 2
7 6
8 0
```

```ltspice
.tran 100 uic
单位阶跃函数
B
V=u(time-10)
sinc
B
V=sin(time-50)/(time-50)
正弦
B
I=sin(time*2)
包络
R = 20
c IC = 1v
衰减振荡
B
V=v(包络)*v(正弦)
```

```txt
电压源V
电流源I
流控电压源H
压控电压源E
流控电流源F
压控电流源G
```
