## linux  
  
[Linux零基础快速入门](https://www.bilibili.com/video/BV1n84y1i7td/?spm_id_from=333.337.search-card.all.click)  
需要看到ssh连虚拟机,就可以停了,然后看他们的[PPT文档](https://pan.baidu.com/s/19hRg2LI1AuqXTQL9IoG5dw?pwd=9987 )  
[CentOS download](https://mirrors.aliyun.com/centos/7/isos/x86_64/CentOS-7-x86_64-DVD-2009.iso)  
阿里云:centos→下载地址→找到并点击“7/”→找到并点击“isos/”→“x86_64/”→找到4.4GB大小的镜像文件并点击开始下载  
  
### Linux系统常用单词翻译  

>
> - new folder 新建文件夹  
paste 粘贴  
select all  全选  
open in terminal  打开终端/命令行  
keep aligned   保持对齐  
organize desktop by name按名称组织桌面  
change background更改背景  
cancel 取消  
create创造 创建  
wallpapers 壁纸  
notificcations通知  
search搜索  
region区域  
univetsal access通用存取  
online accounts联机帐户  
privacy隐私  
sharing共享  
sound 声音  
power权力 权限  
network网络  
lock screen锁频  
open in new tab在新标签页中打开  
open in new window 在新窗口中打开  
cut 剪切  
copy 复制  
move to移动到  
copy to复制到  
move to trash移到垃圾箱 删除  
resize icon调整图标大小  
rename重命名  
compress压紧 压缩  
properties 属性  
  
#### 二.学习笔记  
  
> - 命令:命令本体command+选项,控制命令的行为细节［-options］+参数,控制命令的指向目标(parameter)  
> - ls命令,作用是列出目录下的内容,语法如下:ls[-a-l-h] [linux路径]  
ls -l -a  
ls -la  
ls -al  
三种写法都是一样的,同时应用-l(竖向排列展示内容)和-a(列出所有文件夹,包含隐藏的)功能  
> - cd命令 切换工作目录  
语法:cd+［linux路径］参数  
> - pwd 查看当前工作目录  
语法:pwd  
绝对路径:以根目录为起点,路径描述以/开头  
写法:cd /home/ittangmao/Desktop  
相对路径:以当前目录为起点  路径描述无需以/开头  
写法:cd Deaktop  
> - madir命令,创建新目录(文件夹)  
语法:mkdir [-p] LIinux路径(参数必须填写)  
> - touch命令创建文件  
语法:touch ［Linux］路径  
> - cat查看文件内容  
语法:cat linux路径  
> - more 查看文件内容 可以翻页查看(按空格翻页 按q退出查看)  
语法:more linux路径  
> - cp 复制文件或者文件夹  
复制文件 cp 参数1(复制的文件内容) 参数2(复制去的地方)  
复制文件夹 cp -r 参数1 参数2  
> - mv 移动文件或者文件夹  
语法:mv (-r) 参数1 参数2  
> - rm删除文件 文件夹  
语法:rm [-r -f] 参数1 参数2 …参数N  
-r 用于删除文件夹  
-f 用于强制删除  
> - su - root 输入密码123456(默认的)  
临时切换到root用户  
> - exit 切换回普通用户  
用户组管理 需要root权限  
groupadd 用户名(创建用户组)  
groupdel 用户名(删除用户组)  
getent group 查看当前系统有多少用户组  
认知权限信息:  
r (read)查看权限 w (write)修改权限  
x (execute)执行权限  
> - chmod 命令 修改文件 文件夹 的权限信息(只有文件,文件夹 所属用户或者root用户可以修改)  
语法:chmod ［-R］权限 文件或文件夹  
例:chmod u＝rwx,g＝rx,o＝x hello.txt(将文件权限改为:rwxr-x--x)  
例:chmod-R u＝rwx,g＝rx,o＝x test(将文件夹test以及内容全部权限设置为rwxr-x---x)  
r记为4 w记为2 x记为1,可以有:  
​       0:无任何权限,即 ---  
​       1:仅有x权限,--x  
​       2:仅有w权限,-w-  
​       3:有w和x权限,-wx  
​       4:仅有r权限,r--  
​       5:有r和x权限,r-x  
​       6:有r和w权限,rw-  
​       7:有全部权限:rwx  
​       所以751表示rwx(7)r-x(5) --x(1)  
> - chown命令 修改文件 文件夹的所属用户和用户组 ,只适用root用户执行  
语法:chown [-R] ［用户］［:］［用户组］文件或文件夹  
-R,用户,用户组 都是选项  
-R,对文件夹全部内容应用相同规则  
:用于分隔用户和用户组  
如:chown root hellow.txt(将hello.txt所属用户修改为root)  
chown :root hellow.txt(将helow.txt所属用户组修改成root)  
chown root:ittangmao hellow.txt(将hellow.txt所属用户修改为root,用户组修改为ittangamao)  
chown -R root test (将文件夹test所属用户修改为root并对文件夹内全部内容应用同样规则  
(总结,无: 只改所属用户,:右边有名称,改用户组,左右都有: 用户,用户组都改)  
Linux快捷键:  
ctrl+c 强制停止程序,退出命令输入  
ctrl+d 退出或登出(不能用于vi/vim)  
history 查看历史命令  
history ｜grep ch(在历史记录中过滤带有ch命令)  
！+命令的前缀,自动匹配上一次匹配的前缀命令  
ctrl+r 输入内容去匹配历史命令  
光标移动快捷键:  
ctrl+a 调到命令开头  
ctrl+e  跳到命令结尾  
ctrl+键盘左键,向左跳一个单词  
ctrl+键盘右键,向右跳一个单词  
ctrl+l或者clear 清空终端内容(清屏)  
> - yum命令 RPM(安装包)软件管理器,用于自动化安装配置Linux软件  
语法:yum [-y] ［install ｜remove ｜search］软件名称  
-y,自动确认,无需手动确认安装 卸载  
instal安装 remove卸载 search搜索  
(yum命令需要root权限 需联网)  
systemctl命令控制:启动、停止、开机自启能够被syatemctl管理的软件,一般也称之为服务  
语法:systemctl start ｜ stop｜ status ｜enable ｜disable 服务名  
(启动/停止/查看状态/开启开机自启/关闭开机自启)  
NetworkManager 主网络服务  
network 副网络  
firewalld 防火墙  
sshd,ssh服务(Finalshell远程都玩Linux使用的就是这个服务)  
> - ln命令 创建软连接(类似wind系统的快捷方式)  
语法:ln -s 参数1 参数2  
-s 创建软连接  
参数1:被链接的文件或文件夹  
参数2:要链接去的目的地  
如:ln -s /etc/yum.conf ～/yum.conf  
ln -s /etc/yum ～/yum  
> - date命令 查看系统的时间  
语法:date [-d] [+格式化字符串]  
-d 按照给定的字符串显示日期,一般用于日期计算  
格式字符串:  
%Y 年 %y年份的后两位数(00..99)  
%m月份(01..12) %d日(01..31)  
%H小时(00..23)  
%M分钟(00..59)  
%S秒(00.60)  
%s 自1970-01-01 00∶00∶00 UTC到现在的秒数  
修改时区:  
先切换root权限  
rm -f/etc/localtime (删除本地时间)  
ln-s    /usr/share/zoneinfo/Asia/shanghai/etc/locatime (将这个文件链接为本地时间)  
ntpdate -u ntp.aliyun.com(在阿里云网站配合ntp程序实时校准系统时间,需root权限)  
127.0.0.1 这个ip地址指代本机  
0.0.0 特殊ip地址  
可用于指代本机  
也可以在端口绑定中用来确定绑定关系  
在io地址限制中 表示所以ip  
> - hostname 查看主机名字  
hostnamectl set-hostname 新名字  
(更改主机名字,需要root)  