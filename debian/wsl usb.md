# wsl usb

# shell(windows) 管理员
winget install --interactive --exact dorssel.usbipd-win
usbipd list
usbipd bind --busid=<ID>
usbipd attach --wsl --busid=<ID>

# bash(wsl)
sudo apt-get install -y gdb-arm-none-eabi usbip  usbutils
sudo modprobe usbip-core usbip-host
lsusb
sudo usermod -aG dialout $USER
groups

# bash(wsl)
sudo tee -a ~/.bashrc > /dev/null << 'EOF'
# 绑定 USB 设备到 WSL
if [ -z "$(usbip list -r $WSL_HOST_IP 2>/dev/null | grep '2-1')" ]; then
    powershell.exe -Command "usbipd attach --wsl --busid=2-1"
fi
EOF
source ~/.bashrc