# [wsl](https://learn.microsoft.com/zh-cn/windows/wsl/install)

以管理员身份运行 shell

```shell
# 关闭 Hyper-V 相关功能（如果不需要虚拟机）
Disable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-All

# 关闭虚拟机平台（临时禁用，后续会重新启用）
Disable-WindowsOptionalFeature -Online -FeatureName VirtualMachinePlatform

# 关闭旧版 WSL 功能（后续会重新启用）
Disable-WindowsOptionalFeature -Online -FeatureName WindowsSubsystemForLinux

# 关闭 Windows 沙盒（可选）
Disable-WindowsOptionalFeature -Online -FeatureName Containers-DisposableClientVM

# 关闭容器服务（可选）
Disable-WindowsOptionalFeature -Online -FeatureName Containers
```

```shell
# 重启计算机
Restart-Computer -Force
```

启用 WSL 必需功能

```shell
# 启用 WSL 核心功能
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux

# 启用虚拟机平台（WSL2 必需）
Enable-WindowsOptionalFeature -Online -FeatureName VirtualMachinePlatform

# 启用 Hyper-V 基础功能（仅 WSL2 需要，但可不启用完整 Hyper-V）
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All
```

```shell
# 重启计算机
Restart-Computer -Force
```

```shell
wsl --update # 不稳定 很慢
https://github.com/microsoft/WSL/releases # 离线安装版,需梯子
wsl --update --web-download # 通过github 会好许多
wsl --set-default-version 2
# 需要登陆外网
wsl --list --online
wsl.exe --install -d debian
# 已安装的发行版
wsl -l -v
```

```shell
wsl --shutdown
wsl --unregister <发行版名称>  # 替换为你的发行版
```

```shell
# [Debian软件源](https://mirrors.tuna.tsinghua.edu.cn/help/debian/)
# delete /etc/apt/sources.list
sudo rm /etc/apt/sources.list
# write to  /etc/apt/sources.list.d/debian.sources
# echo -e 多行
# sudo tee 覆盖写入
# sudo tee -a 追加写入
# > /dev/null 忽略输出
sudo tee /etc/apt/sources.list.d/debian.sources > /dev/null << 'EOF'
Types: deb
URIs: http://mirrors.tuna.tsinghua.edu.cn/debian
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

sudo apt update -y
# 证书
sudo apt install apt-transport-https ca-certificates

# https 替换为http 保存 ,使用 sed 进行文本替换 -i 选项表示直接修改文件
# s|http://|https://|g 是将所有 http:// 替换为 https:// 的替换命令
sudo sed -i 's|http://|https://|g' /etc/apt/sources.list.d/debian.sources

sudo apt update -y && sudo apt upgrade -y






