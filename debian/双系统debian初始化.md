# Rufus（windows)或 balena-etcher-electron（linux)通过debian DVD镜像制作启动盘
# 安装debian,至少20个G,放在固体硬盘中,free的空间，安装基础和桌面.

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

# 清华源证书
sudo apt install -y apt-transport-https ca-certificates

sudo apt update
sudo apt install -y sudo
sudo usermod -aG sudo wsy

sudo sed -i 's|http://|https://|g' /etc/apt/sources.list.d/debian.sources
sudo apt update -y 

# delete not need software
sudo apt-get remove gnome-games gnome-weather gnome-clocks gnome-music  gnome-contacts gnome-sound-recorder gnome-2048 gnome-chess gnome-klotski gnome-mahjongg gnome-mines gnome-tetravex gnome-nibbles gnome-robots gnome-sudoku gnome-taquin  gnome-chess gnome-maps tali quadrapassel rhythmbox four-in-a-row lightsoff swell-foop hitori cheese five-or-more gnome-calendar gnome-video-effects aisleriot iagno totem shotwell
# search
dpkg --list | grep video | awk '{print $2}'
sudo apt-get autoremove
sudo apt-get clean
# upgrade
sudo apt upgrade

sudo dpkg -i Clash.Verge_2.0.2_amd64.deb
# https://cloud.ykkk.tech/api/v1/client/subscribe?token=154ef8ae3280bf11974bb1f55bdb17d7

# tiemshift
sudo apt install timeshift -y

# https://zhuanlan.zhihu.com/p/675895900
sudo apt purge fcitx* ibus* -y
sudo apt install fcitx5 fcitx5-chinese-addons -y

# https://linuxcapable.com/install-vscodium-on-debian-linux/
sudo apt install -y dirmngr software-properties-common apt-transport-https curl 
curl -fSsL https://gitlab.com/paulcarroty/vscodium-deb-rpm-repo/raw/master/pub.gpg | sudo gpg --dearmor | sudo tee /usr/share/keyrings/vscodium.gpg >/dev/null
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/vscodium.gpg] https://download.vscodium.com/debs vscodium main" | sudo tee /etc/apt/sources.list.d/vscodium.list
sudo apt update
sudo apt install codium -y

# https://developer.aliyun.com/article/1143167
sudo apt install locales -y
sudo dpkg-reconfigure locales 
# zh_CN.UTF-8 UTF-8
sudo tee -a ~/.bashrc > /dev/null << 'EOF'
export LANG=C.UTF-8
EOF
source ~/.bashrc


# vscodium pingin
# gitlens codelldb continue cortex-debug crates even-better-toml rust-analyzer
# /home/wsy/.continue/config.json
# firfox pingin 
# bitwarden dark reader Immersive Translate
# https://password.cdchiyun.com/
# 18908288720@163.com
# wsynzqwhxhny.

# [Flatpak](https://linuxcapable.com/how-to-install-flatpak-on-debian-linux/)
sudo apt install flatpak 
sudo flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
sudo reboot
sudo apt install gnome-software-plugin-flatpak
# search app in [flathub](https://flathub.org/)
# [kicad](https://flathub.org/apps/org.kicad.KiCad)
# debian gome 需要点开软件，点击已安装，等到升级完成后，才能使用flatpak. 通过flathub网上下载的文件，双击后进入软件安装界面，这样安装比用指令行网络稳定

# i find firefox is not stable in useing, so choose chrome or chromium
# i find platpak install chrome or chromium is not stable in useing, so install chromium in apt
# https://linuxcapable.com/how-to-install-chromium-browser-debian-linux/
# sudo apt install chromium
# chromium

# flatpak 的权限管理
# https://flathub.org/apps/com.github.tchx84.Flatseal 

# https://linuxcapable.com/how-to-install-google-chrome-on-debian-linux/
sudo apt install software-properties-common apt-transport-https ca-certificates curl -y
curl -fSsL https://dl.google.com/linux/linux_signing_key.pub | sudo gpg --dearmor | sudo tee /usr/share/keyrings/google-chrome.gpg >> /dev/null
echo deb [arch=amd64 signed-by=/usr/share/keyrings/google-chrome.gpg] http://dl.google.com/linux/chrome/deb/ stable main | sudo tee /etc/apt/sources.list.d/google-chrome.list
sudo apt update
sudo apt install google-chrome-stable





