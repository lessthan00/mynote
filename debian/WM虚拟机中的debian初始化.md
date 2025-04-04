# debian

### 阿里云中的debian 最新DVD镜像下载，和校验。
### [vmware](https://www.52pojie.cn/thread-1986577-1-1.html)
### debian 初始化设置，安装软件 ssh web standard xfce 

### debian get ip and hostname

#!bash
hostname -I
hostname 


## vscode remote-ssh connect

ssh 远程密钥登录

#！shell
ssh-keygen -t rsa -b 4096 -C "mail" -f C:\Users\$(user)\.ssh\id_rsa_user_debian
ssh-keygen -t rsa -b 4096 -C "mail" -f C:\Users\$(user)\.ssh\id_rsa_root_debian
scp C:\Users\$(user)\.ssh\id_rsa_user_debian.pub $(user)@$(ip):~/
scp C:\Users\$(user)\.ssh\id_rsa_root_debian.pub $(user)@$(ip):~/
ssh $(user)@$(ip)


#!bash
mkdir -p ~/.ssh
chmod 700 ~/.ssh
cat ~/id_rsa_user_debian.pub >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys


# vscode Remote-SSH config

```shell
Host debian
    HostName 192.168.0.103
    User $(name)
    IdentityFile ~/.ssh/id_rsa_user_debian
    RequestTTY yes
    RemoteCommand sudo /bin/bash 

Host debian-root
    HostName 192.168.0.103
    User root
    IdentityFile ~/.ssh/id_rsa_root_debian
    RequestTTY yes
    RemoteCommand /bin/bash
```


# [Debian软件源](https://mirrors.tuna.tsinghua.edu.cn/help/debian/)
# delete /etc/apt/sources.list
sudo rm /etc/apt/sources.list
# write to  /etc/apt/sources.list.d/debian.sources
# echo -e 多行
# sudo tee 写入
# > /dev/null 忽略输出
sudo tee /etc/apt/sources.list.d/debian.sources > /dev/null << 'EOF'
Types: deb
URIs: https://mirrors.tuna.tsinghua.edu.cn/debian
Suites: bookworm bookworm-updates bookworm-backports
Components: main contrib non-free non-free-firmware
Signed-By: /usr/share/keyrings/debian-archive-keyring.gpg
# 默认注释了源码镜像以提高 apt update 速度，如有需要可自行取消注释
# Types: deb-src
# URIs: https://mirrors.tuna.tsinghua.edu.cn/debian
# Suites: bookworm bookworm-updates bookworm-backports
# Components: main contrib non-free non-free-firmware
# Signed-By: /usr/share/keyrings/debian-archive-keyring.gpg
# 以下安全更新软件源包含了官方源与镜像站配置，如有需要可自行修改注释切换
# Types: deb
# URIs: https://security.debian.org/debian-security
# Suites: bookworm-security
# Components: main contrib non-free non-free-firmware
# Signed-By: /usr/share/keyrings/debian-archive-keyring.gpg
# Types: deb-src
# URIs: https://security.debian.org/debian-security
# Suites: bookworm-security
# Components: main contrib non-free non-free-firmware
# Signed-By: /usr/share/keyrings/debian-archive-keyring.gpg
EOF

# 清华源证书
sudo apt install -y apt-transport-https ca-certificates

sudo apt update
sudo apt install -y sudo
sudo usermod -aG sudo $(user)
/sbin/reboot

sudo sed -i 's|http://|https://|g' /etc/apt/sources.list.d/debian.sources
sudo apt update


mkdir -p /root/.ssh
chmod 700 /root/.ssh
touch /root/.ssh/authorized_keys
cat /home/$(user)/id_rsa_root_debian.pub >> /root/.ssh/authorized_keys
chmod 600 /root/.ssh/authorized_keys
tail /root/.ssh/authorized_keys
rm /home/$(user)/id_rsa_root_debian.pub 


静态ip

bash
sudo nano /etc/network/interfaces
# The primary network interface
auto eth0
iface eth0 inet static
address 192.168.0.103
netmask 255.255.255.0
gateway 192.168.0.1
dns-nameservers 8.8.8.8 8.8.4.4

sudo systemctl restart networking
ip a
sudo reboot



