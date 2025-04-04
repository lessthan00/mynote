前言
本文章仅对使用过其他电路设计软件的开发人员有参考价值, 新手推荐在某站上观看某某教育的Cadence17.4零基础入门

Capture软件->原理图部分
原理图常用快捷键:
Esc 结束走线等操作
I 放大
O 缩小
C 以光标所指为新的窗口显示中心
W 画线
P 快速放置元件
H 元件标号左右翻转
V 元件标号上下翻转
R 元件旋转 90°
N 放置网络标号
J 放置节点
F 放置电源
G 放置地
Y 画多边形
T 放置 TEXT
B 放置总线
E 放置总线端口
T 放置 TEXT
操作界面和常用设置介绍
颜色设置:options->perference->Color/Print
格点设置:options->perference->Grid Display
Sechematic Page Grid:设置原理辑器格点
Part and Symbol Grid:设置元器件编辑器格点
点击Snap To Grid（工具栏图标）->打开/关闭 抓取格点
软件自带元件库介绍
路径:安装目录/cadence/spb17.4/tools/capture/library/*.olb
Amplifier.olb:模拟放大器IC
Arithmetic.olb:逻辑运算IC
ATOD.OLB:A/D转换IC
BusDriverTransceiver.olb:汇流排驱动IC
capsym.olb:电源、地、输入输出、标题栏等
Connector.olb:连接器
Counter.olb:计数器IC
Discrete.olb:分立式元件, 如电阻、电容、电感、开关、变压器等常用器件
DRAM.OLB:动态存储器
ElectroMechanical.olb:马达、断路器、等电机类
FIFO.OLB:先进先出资料暂存器
Filter.olb:逻辑门（含TTL/CMOS）
FPGA.OLB:可编程逻辑器件
Gate.olb:逻辑门
Latch.olb:锁存器
LineDriverReceiver.olb:线控驱动器与接收器
Mechanical.olb:机构图件
MicroController.olb:单晶片微处理器
MicroProcessor.olb:微处理器
Misc.olb:杂项, 电表、微处理器周边未分类的的零件
Misc2.olb:杂项图件
Misc3.olb:
MiscLinear.olb:线性杂项图件
MiscMemory.olb:记忆体杂项图件
MiscPower.olb:高功率杂项图件
MuxDecoder.olb:解码器
OPAmp.olb:运放
PassiveFilter.olb:被动式滤波器
PLD.OLB:可编程逻辑器件
PROM.OLB:只读记忆体运放
Regulator.olb:稳压IC
ShiftRegister.olb:移位寄存器
SRAM.OLB:静态存储器
Transistor.olb:晶体管（含FET、UJT、PUT等）
创建元器件库
方法一:手动添加引脚元素
File->New->Library->OK
工程栏右键刚刚新建的Library->Save As…->将新建的符号库另存
工程栏右键刚刚新建的Library->New Part
添加单个引脚->Place Pin（工具栏图标）->添加属性:
属性	解释
Name	引脚名
Number	引脚编号
Shape	特征
Type	类型
Pin#Increment for Next Pin	引脚编号增量
添加多个引脚->Place Pin array（工具栏图标）->添加属性:
属性	解释
Starting Name	起始引脚名
Starting Number	起始引脚编号
Number of Pins	总引脚
Pin Spacing	间距:单位是格点
Shape	特征
Type	类型
Pin#Increment for Next Pin	引脚编号增量
->点击空白处->Property Sheet（窗口）->Edit Pins->Name->编辑引脚名
6. 添加图形->place->选择图形绘制

方法二:复制粘贴
打开Design Cache（工程窗口）, 或者需要复制元件所在的Library（工程窗口）
右键需要复制的元件Copy
右键需要复制到的Library（工程窗口）->Paste
库管理和调用
将库添加到工程->右键Library（工程窗口）->Add File
元器件库与PCB封装库关联介绍
双击符号器件, 关联封装由 PCB Footprint 参数决定, 如下:

![img](https://img-blog.csdnimg.cn/direct/97684262f8974f59a12c7c22825f4138.png#pic_center)

信号连接
导线连接:仅同一页面
网络标号连接:仅同一页面
分页符号（离图连接）连接:支持多页面连接
电源、地连通
系统自带电源/地连接符的库
电源/地连接符为全局变量、多页面连接
修改电源/地连接符的Name, 即可联通
添加差分属性
鼠标左键选中一页原理图->Tools（菜单栏）->Create Differential Pair…->

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/ba9ad12a261440f29ddb39395aaeaf0b.png#pic_center)

DRC检测工具
运行DRC检查:PCB（菜单栏）->Design Rules Check…->Run
设置DRC检查规则:PCB（菜单栏）->Design Rules Check…->Rules Setup、Report Setup
PDF与BOM清单输出
输出PDF
File（菜单栏）->Export->PDF
File（菜单栏）->Print->Setup（打印设置）->
输出BOM:Tools（菜单栏）->Bill of Materials:

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/2aed48571ed7497593c19b393425b75f.png#pic_center)

原理图输出Allgro第一方网表及常见错误解析
生成网表:Create Netlist（工具栏图标）->确定
常见问题分析:出现错误将会弹出Error窗口, 在netlist.log文件中有详细信息
Duplicate Pin Name “xxx” found on Package … ->引脚重复名:在Orcad中只有电源引脚名称允许重复
Property “PCB Footprint” missing for instance xxx ->封装名缺失
Pin number missing from “x” of Pacage TEST … ->管脚编号缺失
Value for property PCB Footprint contains carriage return for xxx ->属性名称中含有回车键
Conflicting Value of following Component Defintion property found on different sections of xxx … ->多Part器件属性冲突:将属性修改一致
Illegal charater in xxx … ->非法字符
PCB Eidtor->PCB部分
PCB封装库的管理与调用
文件介绍:

焊盘文件->***.PAD
封装文件->***.DRA（可编辑文件）;
***.PSM（调用文件）
路径指定:Setup（菜单栏）->User Perference…->

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/85efe2c500744159ac08d71818c3ade8.png#pic_center)

调用封装:Place（菜单栏）->Manually…->Advanced Settings中勾选上 Library ->Placement List->下拉框中选择 Package Symbols->放置封装

导出封装:File（菜单栏）->Export->Libraries…->全部勾选->Export

快捷键设置、Capture与PCB Eidtor同步网表
快捷键设置
alias:输入后需要回车
funckey:输入后直接执行
env文件路径->…\Cadence\SPB17.4\share\pcb\text
几个不要定义快捷的字母和按键:F1 X Y i
同步网表
导入网表->File（菜单栏）->Import->Logic/Netlist…

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/074f7cbc7ba34558aa8e87d9370766fb.png#pic_center)

查看状态->Display（菜单栏）->Status…

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/54d9736c0ed04916815c0831467fa512.png#pic_center)

后台放置元器件
Place（菜单栏）->QuickPlace…->Place->OK

手工添加元器件与网络修改
打开逻辑编辑使能->Setup（菜单栏）->User Perference…->Logic->勾选 logic_edit_enabled->Aplly
Logic（菜单栏）->Part Logic…->

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/d23fc5b50aa0469b9807079f730b4c5c.png#pic_center)

Place（菜单栏）->QuickPlace…->Place->OK

3. 修改网络:Logic（菜单栏）->Net Logic…->在Options窗口选择网络->点击需要修改网络的Pin
4. 删除:Logic（菜单栏）->Part Logic…->选择需要删除的器件->Delete

DXF文件导入与PCB板框定义
板框层:16以前的在Board geometry.Outline, 17版本在Board geometry.Design_Outline

Options窗口选择到Board geometry.Design_Outline->Shape（菜单栏）选择图形->绘制板框

DXF导入:

File（菜单栏）->Import->DXF…

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/a954f73db66949d98832b6bbdb22a800.png#pic_center)

Edit/View Layers…参数:

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/7add3037d3e54e7f8a9f3a17c9c78041.png#pic_center)

Edit（菜单栏）->Change->选择（属于Shapes）->Options窗口中选择Board geometry.Design_Outline->右键Done
精准定位与坐标定位、Capture与PCB Eidtor交互式布局
定位
精准定位:在Move命令下->选择定位点->右键Snap pick to->选择抓取类型
坐标定位:在Move命令下->在Command窗口输入坐标->x (x坐标信息) (y坐标信息)
交互式布局
PCB Eidtor需要使用第一方网表
Capture中->Options（菜单栏）->Perference->Miscellaneous->勾选使能Enable Intertool Communication
网络飞线的常规处理方式
打开/关闭飞线->Rats All（工具栏图标）/Unrats All（工具栏图标）
隐藏/显示某一网络飞线->Display（菜单栏）->Show Rats/Blank Rats->Net->选择网络
器件移动、旋转、镜像、对齐、等间距, 锁定与解锁
移动、旋转->Move命令->选目标->右键Rotate（旋转）/Mirror（镜像）

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/2e451cb088264b0ca28e168d98d9faf8.png#pic_center)

器件镜像（换层）->Mirror命令->选目标->选择镜像参考点

对齐、等间距->打开布局模式->Setup（菜单栏）->Application Mode->Placement Edit->框选元件->右键Align components->Options对齐参数如下:

Horizontal:水平方向对齐
Vertical:垂直方向对齐
Top:顶部对齐（Horizontal模式）;Left:左对齐（Vertical模式）
Center:中心对齐
Bottom:底部对齐（Horizontal模式）;Right:右对齐（Vertical模式）
Equal spacing:对齐的时候进行等间距
锁定:Fix（工具栏图标）->选择对象

解锁:Unfix（工具栏图标）->选择对象

相同模块复用
选择布局模式:Setup（菜单栏）->Application Mode->Placement Edit

框选元件、走线、过孔、铜皮等模块元素->右键->Place replicate create->右键Done->左键选择中心点->保存模型（和PCB放在同一位置）->Save

使用模块->框选元件->右键->Place replicate apply->选择之前保存的模型

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/c336ce2b5c44488187391ca86105a85e.png#pic_center)

点击OK完成

测量、查询
Show measure（工具栏图标）、Show Element（工具栏图标）
几个名词解释
Dist:中心到中心的距离
Total Dist:几次的Dist之和
Ari Gap:边缘距离（中心出了焊盘到另一中心出了焊盘的距离）
Dx,Dy:X轴, Y轴长度, Manhattan Dist = Dx + Dy
Pick Angel:角度
OVERLAP:重叠
开启第二种单位:Setup（菜单栏）->User Perference->Display->Element->showmeasure altunits->选择第二种单位->Apply->OK
PCB布线叠层与阻抗设计
叠层:Xsection（工具栏图标）

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/58bdb0cc2b91477c8b84d9eafe1fd251.png#pic_center)

PCB布线过孔添加与设置
放置过孔:走线时双击左键（在Options窗口中可以选择已经添加的过孔）
添加过孔:Setup（菜单栏）->Constraints->Constraint Manager…->

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/0ece994745b14d99b797007e49172e53.png#pic_center)

差分对添加与设置
手动添加:Logic（菜单栏）->Assign Differential Pair/删除:选择差分对, 点击delete

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/4562bcf2536345a4b612e19ebf6a27da.png#pic_center)

规则管理器添加:打开规则管理器->Cmgr（工具栏图标）->Physical->Net->All Layers-选择差分网络右键->Create->Differential Pair->新窗口中点击Create
删除:打开规则管理器->Cmgr（工具栏图标）->Physical->Net->All Layers-选择差分网络右键->右键delete

模糊添加:Logic（菜单栏）->Assign Differential Pair

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/cde2f738661f433482c9b84f1e1ee5d9.png#pic_center)

规则间距添加
布线规则添加:打开规则管理器->Cmgr（工具栏图标）->

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/2b7298958b4d41ee8f7feaf51ac7ba7e.png#pic_center)

布线规则驱动:打开规则管理器->Cmgr（工具栏图标）->Physical->Net->All Layers->Referenced Physical Cset（在该列中选择Net对应的规则）

间距规则添加:打开规则管理器->Cmgr（工具栏图标）->

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/a4e5cc9de339420d8a93b4c196805027.png#pic_center)

间距规则驱动:打开规则管理器->Cmgr（工具栏图标）->Spacing->Net->All Layers->Referenced Physical Cset（在该列中选择Net对应的规则）

Class添加和颜色分配
Net-Class:线宽、线距的信号集合
Net-Group:某类信号的集合（16.6以前为Bus）
Match-Group:某类信号的等长参数
创建Net-Class/Net-Group:打开规则管理器->Cmgr（工具栏图标）->

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/280f619e9504456fbb615edf2aa92148.png#pic_center)

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/e81b69dcfa2d4f9b89835c33aa5b8762.png#pic_center)

->在NetClass中命名->勾选Creat fot both physical and spacinng（Net-Group无该选项）->Ok

2. 目标颜色分配:Display（菜单栏）->Assign color
3. 高亮/低亮:Display（菜单栏）->Highlight/Rehighlight->（Options窗口中可以选择样式, 需要在User Perferences Editor中的Highlight栏里取消勾选display_nohilitefont）->选择对象（Rehighlight在Options窗口中取消勾选Retain object custom color;默认取消勾选方法:在User Perferences Editor中的Highlight栏里勾选disable_Retain_object_custom_color）
4. 各层颜色设置:color192（工具栏图标）
5. 导入配色方案:File（菜单栏）->Import->Parameters…

区域规则添加
添加规则:打开规则管理器->Cmgr（工具栏图标）->添加规则, 与26节中的步骤一样 ->添加区域:

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/c120bc46c5e64989843fc0837b9a3efb.png#pic_center)

添加区域:shape（菜单栏）->选择绘制的图形->Options窗口中选择Constraint Region.Top/Bottom(子类中选择层), Assign to Region中选择规则->绘制区域

走线、修线
走线:Add connect（工具栏图标）->Options窗口选项讲解

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/88e1615c08374e6bb9cd9ea4a5da1816.png#pic_center)

修线:Slide（工具栏图标）->Options窗口选项讲解

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/b69499eccfbb4c6d95ccb7fe0e821ab6.png#pic_center)

复制、改变、删除
Copy（同一Class）:Edit（菜单栏）->Copy->Options面板中选择参考点:1.Symbol（器件）2.Body Center（器件中心）3.User Pick（选取一个参考点）4.Sym Pin（器件焊盘）->选取元素后->Options面板可选择矩形/极坐标粘贴的参数, 是否保留网络
Z-Copy（必须是封闭图形, 不同Class, 通常用来建立KeepIn/keepOut）:Edit（菜单栏）->Z-Copy->Options参数设置->几个名词解释Contract（内缩）、Expand（外扩）
修改:
改变线宽:Edit（菜单栏）->Change->Options窗口只勾选Line Width并输入参数, 并在Find窗口勾选可选中元素
改变文本大小:
Setup（菜单栏）->Design Parameter Editor->Text->Setup text size中添加/设置/字体大小种类->OK
Edit（菜单栏）->Change->Options窗口只勾选Text block并选择字体种类->选择修改元素
改变Class（只能修改SubClass）:Edit（菜单栏）->Change->Options窗口只勾选New Subclass->选择修改元素
改变文本内容化:Text Edit（工具栏图标）->选择修改元素
删除: Edit（菜单栏）->Delete->选择删除元素
Sub-Drawing（不同PCB间的复制）
在PCB-A中->Files（菜单栏）->Export->Sub-Drawing->选择元素（可以右键Team Group）->右键Complete->Command中输入坐标（原点坐标输入:x 0 0）->选择导出路径->保存
在PCB-B中->Files（菜单栏）->Import->Sub-Drawing->选择文件->Command中输入坐标（原点坐标输入:x 0 0）回车
铜箔
参数设置:Shape（菜单栏）->Global Dynamic Shape Parameters（Layer Dynamic Shape Parameters是对单层进行设置, 一般不用）:

Shape fill:

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/4a8b0eabb5a3450dafbfedcffbc5e88e.png#pic_center)

Void controls:

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/022a4942d14c45e0898fe24eb0b6b910.png#pic_center)

Clearances:

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/d06751b7c9724e62bfabb7b73794fdf2.png#pic_center)

Thermal relief connects:

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/91b5ccbd431e40df8cfd23a03352f496.png#pic_center)

放置铜皮
Shape（菜单栏）->选择图形->Options面板参数设置:

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/e0cc453fba804fc6a5f0e85992908041.png#pic_center)

如果没有指定网络:Shape（菜单栏）->Select Shape or Void/Cavity->选择铜皮->右键Assign Net->左键点击网络

单独铜皮设置参数:Shape（菜单栏）->Select Shape or Void/Cavity->选择铜皮->右键Paramteres

挖铜/避让（只针对覆铜有效）:Element是针对静态铜皮, 需要对静态铜皮的Parameters中Clearances进行设置

Shape（菜单栏）->Manual Void/Cavity->选择图形/（Element是添加避让）->在已有铜皮上绘制（Options窗口参数设置和覆铜一样）
删除/移动/复制: Shape（菜单栏）->Manual Void/Cavity->Delete/Move/Copy->选择被 挖铜/避让 的铜皮->选择手动添加的 挖铜/避让 区域
修铜:Shape（菜单栏）->Edit Boundary->选择铜皮编辑

动态和静态铜皮的转换:Shape（菜单栏）->Change Shape Type->Options选择参数->选择需要转换的铜皮

删除孤岛铜皮（死铜）:Shape（菜单栏）->Delete Islands

铜皮分割:

Add（菜单栏）->Line->Options中选择层->画分割带
添加倒角:Manufacture（菜单栏）->Drafing->Chamfer(直角)/Fillet(圆弧)

PCB布线状态查验及相关DRC模型设置与消除
状态查询:Display（菜单栏）->Status…

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/fcfc05b3c82c4d189af5fc61da13ad76.png#pic_center)

DRC模型设置:Setup（菜单栏）->Constraints->Modes中设置

标注
Manufacture（菜单栏）->Dimension Environment->进入标准环境

空白出右键Parameters进行参数设置:

General:Standard conformance->一般选择ANSI
Parameter editing->单位选择
Text:

![在这里插入图片描述](https://img-blog.csdnimg.cn/direct/bb980ec01f424425914e5441fbc787b9.png#pic_center)

设置完成后点击右键选择标注类型（标注时配合右键Snap Pick to（抓取）效率高）:

Linear Dimension:线性尺寸标注
Datum Dimension:相对坐标标注
Angular Dimension:角度标注
Leader Line:箭头执行的形式进行标注
Diametral Leader:标注圆的直径
Redial Leader:标注圆的半径
Balloon Leader:圆形标注的指引形式
Chamfer Leader:标注倒角
删除:右键Delete Dimension
————————————————

                            版权声明:本文为博主原创文章, 遵循 CC 4.0 BY-SA 版权协议, 转载请附上原文出处链接和本声明.

原文链接:https://blog.csdn.net/m0_48919895/article/details/138006963