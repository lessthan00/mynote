:: 打开回显
@echo off 
:: 切换路径到要执行的脚本目录
cd /d %~dp0
:: 执行python文件
python kicad2jlc_gerber.py
:: 执行文件后暂停
pause
exit