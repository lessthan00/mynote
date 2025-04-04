```
正则表达式,不贪婪
"Supplier Part" "(.*?)"
```

Common_Library

```
property "SUPPLIER PART" "C
(?=property "SUPPLIER PART" ")(.+)(")
```

kicad文件批量修改

点击正则匹配

.kicad_pcb

```
footprint "
footprint "Common_Library:

property "Footprint" "
property "Footprint" "Common_Library:

检查
property "Reference" "UNK"
.PcbLib:
.IntLib:
gr_text_box
```

.kicad_sch

```
改一个符号的名称需要从
symbol "x:Y"
symbol "Y_0_1"
lib_id "x:Y"
来改.
```



```
from AD
property "Footprint" "
property "Footprint" "Common_Library:

(?=symbol)(.+)(-altium-import:)
symbol "Common_Library:

(?=lib_id)(.+)(-altium-import:)
lib_id "Common_Library:

(?=pin)(.+)(line)
pin passive line

from jlc
(?=property "Footprint" ")(.+)(:)
property "Footprint" "Common_Library:


symbol(.*?)"
(?=symbol)(.+)(:)
symbol "Common_Library:

lib_id(.*?):
lib_id "Common_Library:

(?=pin)(.+)(line)
pin passive line

Manufacturer Part
(hide yes) 删除
```

.sym

```
from jlc
(?=property "Footprint")(.+)(:)
property "Footprint" "Common_Library:

property "Footprint" "
property "Footprint" "Common_Library:

(?=pin)(.+)(line)
pin passive line
```

### 电路板设置

文件(F)->电路板设置

https://www.bilibili.com/video/BV1Ea411K7fg/?spm_id_from=333.999.0.0&vd_source=188a5e02d520f745e2a0cd650b30aa4b

```
MASK 阻焊层
paste 锡膏层
silkscreen 丝印层
adhesive 贴片零件的点胶
Coutyard 元件边界层.
fab 文件信息,位号之类.
edge.cuts 外框层.
margin 带有电气属性的边界

	约束
铜箔
最小间距5mil
最小走线宽度5mil
最小连线宽度5mil
最小孔环宽度4mil
最小过孔直径18mil
铜到孔的间距5mil
铜箔到板边间距5mil
过孔
最小通孔10mil
孔到孔的间距5mil
微孔
最小温控直径8mil
最小位孔孔径4mil
丝印层
最小项目间距0mil
最小文本高度30mil
最小文本线宽3mil
弧线/圆 由线段逼近
最大允许偏差:0.2mil
	预设尺寸
走线宽度:5mil/7mil/11.8mil/20mil/40mil/120mil
过孔:18/9mil 24/12mil
	网络类
```

敷铜

```
优先级: 1>0, 所有一般大面积敷铜使用优先级0,小面积敷铜使用优先级1, 敷铜走线使用优先级3.
```

文本编辑

```
编辑(E)->编辑文本与图形属性
编辑(E)->编辑走线与过孔属性
选择文本->框选->改文本大小和走线粗细.
```


