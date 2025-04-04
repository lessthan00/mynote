import os
import sys

def remove_empty_folders(path):
    """
    删除指定路径下的所有空文件夹。

    :param path: 要清理的文件夹路径
    """
    # 使用 os.walk 遍历文件夹，topdown=False 表示从子文件夹开始遍历（确保先删除子文件夹）
    for root, dirs, files in os.walk(path, topdown=False):
        for dir_name in dirs:
            dir_path = os.path.join(root, dir_name)  # 获取文件夹的完整路径
            try:
                # 检查文件夹是否为空
                if not os.listdir(dir_path):
                    os.rmdir(dir_path)  # 删除空文件夹
                    print(f"Deleted empty folder: {dir_path}")
            except OSError as e:
                # 捕获并打印可能的错误（例如权限问题）
                print(f"Error: {e}")

if __name__ == "__main__":
    # 获取当前工作目录，使程序更具普适性
    # 这样无论将 EXE 文件放在哪个目录下，都会清理该目录下的空文件夹
    folder_path = os.getcwd()
    
    # 调用函数删除空文件夹
    remove_empty_folders(folder_path)
    
    # 防止窗口立即关闭（仅在 Windows 下双击运行时有效）
    # 如果是双击运行 EXE 文件，程序执行完毕后窗口会立即关闭，添加 input() 可以让用户按 Enter 键退出
    if sys.platform == "win32":
        input("Press Enter to exit...")

# 需要构建成一个python的 exe运行程序, 只包含这一个python文件 delete_empty_folder.py 
# 需要获取的目标目录中r"d:\Users\Desktop\江协科技stm32\程序源码\STM32Project-有注释版"并不普适,需要换成普适的,即这个exe所处的目录即使要清理的文件夹的目录
# 同时路径只有外部的,而且相对来说固定
# 而且只需要双击exe即可运行
# 我现在需要优化一下这个代码,使构建的 exe运行更流畅,体积更小,更少出问题, 更有普适性
# 然后我需要构建exe的命令行,我在windows平台,这是需要注意的.
# 请帮我写好优化点的注释

# 构建
# pip install pyinstaller
# pyinstaller --onefile --windowed delete_empty_folder.py
# --onefile：将所有依赖打包成一个单独的 .exe 文件
# --windowed：避免在运行时显示命令行窗口（适合双击运行）
# --clean：清理临时文件和缓存，确保构建过程更干净
# --noconsole：不显示控制台窗口（适用于GUI程序或不需要用户交互的脚本）
# --icon="path_to_icon.ico" 添加一个图标
# [UPX](https://upx.github.io/) --upx-dir=C:\path\to\upx 压缩体积
# 最终
# pyinstaller --onefile --upx-dir=D:\light\upx-4.2.4-win64 delete_empty_folder.py

