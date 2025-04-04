下载debian12的 build-essential:amd64 和 sudo:amd64 的deb及其依赖包，下载到当前文件夹中
到另一个电脑的debian,使用dpkg安装，需要解决安装时的依赖问题。
因为sudo dpkg -i *.deb会报错，因为先安装的其依赖大概还没安装，在后面一点点。
还有需要检测安装已经完成。

```bash
#!/bin/bash

# 定义一个函数来处理指定包的依赖和下载
download_package_deps() {
    local package="$1"
    local dir="$package"

    # 创建目录并进入
    mkdir -p "$dir"
    cd "$dir" || exit

    # 清空依赖文件
    > dependencies.txt

    # 获取包的依赖并写入文件
    apt-cache depends --recurse --no-recommends --no-suggests --no-conflicts --no-breaks --no-replaces --no-enhances "$package":amd64 | grep "^\w" | sort -u >> dependencies.txt

    # 下载所有依赖的 .deb 文件
    for deb in $(cat dependencies.txt); do
        apt-get download "$deb"
    done

    # 清空已下载包记录文件
    > downloaded_packages.txt

    # 记录下载的 .deb 文件
    while IFS= read -r package; do
        deb_file=$(ls | grep -E "^${package}_.*\.deb$")
        if [ -n "$deb_file" ]; then
            echo "$deb_file" >> downloaded_packages.txt
        else
            echo "未找到与 $package 对应的 .deb 文件"
        fi
    done < dependencies.txt

    # 返回上级目录
    cd ..
}

# 调用函数，参数为包名
download_package_deps "wireless-tools"
download_package_deps "sudo"
download_package_deps "wpasupplicant"
download_package_deps "net-tools"
download_package_deps "dpkg"
```

```bash
#!/bin/bash

# 定义一个函数来安装指定目录下的所有deb包
install_debs() {
    local dir="$1"
    cd "$dir" || exit
    for deb in $(tac downloaded_packages.txt); do
        sudo dpkg -i "$deb"
    done
    cd ..
}

# 安装各个目录下的deb包
install_debs "wireless-tools"
install_debs "wpasupplicant"
install_debs "sudo"
install_debs "net-tools"
install_debs "dpkg"

dpkg -l build-essential wireless-tools sudo wpasupplicant dpkg net-tools
/usr/sbin/usermod -aG sudo wsy   
/sbin/reboot
```

```bash
su  
sudo nano /etc/apt/sources.list 
```

```conf
# 阿里镜像源
deb https://mirrors.aliyun.com/debian bookworm main non-free non-free-firmware contrib
deb https://mirrors.aliyun.com/debian bookworm-updates main non-free non-free-firmware contrib
deb https://mirrors.aliyun.com/debian bookworm-proposed-updates main non-free non-free-firmware contrib
deb https://mirrors.aliyun.com/debian bookworm-backports main non-free non-free-firmware contrib
deb https://mirrors.aliyun.com/debian bookworm-backports-sloppy main non-free non-free-firmware contrib
deb-src https://mirrors.aliyun.com/debian bookworm main non-free non-free-firmware contrib
deb-src https://mirrors.aliyun.com/debian bookworm-updates main non-free non-free-firmware contrib
deb-src https://mirrors.aliyun.com/debian bookworm-proposed-updates main non-free non-free-firmware contrib
deb-src https://mirrors.aliyun.com/debian bookworm-backports main non-free non-free-firmware contrib
deb-src https://mirrors.aliyun.com/debian bookworm-backports-sloppy main non-free non-free-firmware contrib

# 阿里安全更新镜像源
deb https://mirrors.aliyun.com/debian-security bookworm-security main non-free non-free-firmware contrib
deb-src https://mirrors.aliyun.com/debian-security bookworm-security main non-free non-free-firmware contrib
```
