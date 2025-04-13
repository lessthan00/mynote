AD19 导出gerber : 制造输出->Gerber X2 files-> 单位毫米 -> 格式4:5-> 绘图仪类型 排序(矢量), 出图层(排除 keep out layer / top pad master / bottom pad master / mechanical 6,13,15), 钻孔(排除 Drill drawing /Drill guide), 排除 Gerber Board profile,并检查 mechanical 1是否已经包含板轮廓信息。
再导出为RS-274X格式,单位毫米,3位整数 5位十进制(小数)，首位取消零,绝对位置。
保存到gerber文件,使用https://github.com/lessthan00/python-code/tree/main/kicad_ad2jlc_gerber,转化为jlc_gerber,去打样.
