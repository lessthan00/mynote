# ssh github
# https://awsm.page/git/use-github-with-ssh-complete-guide-including-vscode-setup/
git config --global user.name "lessthan00"
git config --global user.email "3814983151@qq.com"
git config --global user.name
git config --global user.email

cd ～/.ssh
ssh-keygen -t rsa -b 4096 -C "3814983151@qq.com"

# gihub ->setting -> ssh
ssh -T git@github.com

# 查看更改状态
git status
# 添加到暂存
git add .
# 提交更改
git commit -m "描述你的更改"
# 查看远程仓库状态
git status
# 重命名分支 master -> main
git branch -m master main
# 推送到 main 分支
git push origin main
# 创建并切换到 main 分支
git checkout -b main
