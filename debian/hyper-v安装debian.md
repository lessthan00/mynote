# hypwer-v

## hyper-v 安装debian12

### 准备

windows自带的,搜索启动,打开启动或关闭windows功能,关闭掉冲突的虚拟机平台和适用于linux的windows子系统,打开hyper-v,重启.
重启后打开 hyper-v 管理器, 浏览器搜索 debian dvd iso 阿里云.  
[链接](https://mirrors.aliyun.com/debian-cd/current/amd64/iso-dvd/)
获取DVD
windows商店搜索下载文件批量校验,校验是否下载成功.
vscode可以打开SHA256SUMS

### 虚拟机创建

新建
新建虚拟机
名称debian12
将虚拟机存储到其他位置: C:\Users\admin\Documents
第二代
启动内存4096
不使用动态内存
网络 default switch
硬盘位置:C:\Users\admin\Documents\debianDisk
硬盘大小: 20GB
映像文件:E:\software\debian\debian-12.8.0-amd64-DVD-1.iso
完成

设置
不启用安全启动
虚拟处理器数量 2
应用


