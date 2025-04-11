#  instll docker

debian 操作系统
clash 机场
国内

## [install](https://docs.docker.com/engine/install/debian/)

# test proxy
curl -v https://registry-1.docker.io/v2/

# add proxy

sudo mkdir -p /etc/systemd/system/docker.service.d
sudo tee /etc/systemd/system/docker.service.d/http-proxy.conf <<EOF
[Service]
Environment="HTTP_PROXY=http://127.0.0.1:7897"
Environment="HTTPS_PROXY=http://127.0.0.1:7897"
Environment="NO_PROXY=localhost,127.0.0.1,192.168.0.0/16,10.0.0.0/8,172.16.0.0/12,.internal"
EOF

# resete 
sudo systemctl daemon-reload
sudo systemctl restart docker
sudo systemctl show --property=Environment docker

# add user into docker group
sudo groupadd docker 
sudo usermod -aG docker $USER
newgrp docker
docker run hello-world

