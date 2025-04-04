import os
import shutil
import argparse

def remove_folders(folders_to_remove, base_path):
    for folder in folders_to_remove:
        path = os.path.join(base_path, folder)
        try:
            shutil.rmtree(path)
            if not os.path.exists(path):
                print(f"Successfully removed: {path}")
        except FileNotFoundError:
            print(f"Folder not found: {path}")
        except OSError as e:
            print(f"Error removing {path}: {e}")

def main():
    parser = argparse.ArgumentParser(description="Remove specified folders in AppData\\Roaming.")
    parser.add_argument('--folders', nargs='+', default=["BaiduYunGuanjia", "BaiduYunKernel", "Baidu"],
                        help="List of folders to remove")
    args = parser.parse_args() # 用 argparse 处理命令行参数

    appdata_path = os.getenv('APPDATA') # 使用 os.getenv('APPDATA') 获取普适的 AppData\Roaming 路径
    if not appdata_path:
        print("Could not find the AppData directory.")
        return

    remove_folders(args.folders, appdata_path)

if __name__ == "__main__":
    main()
    
# 需要构建成一个python的 exe运行程序, 只包含这一个python文件 kill_baidu.py 
# 需要获取的目标目录中r"C:\Users\admin\AppData\Roaming"并不普适,需要换成普适的,
# 同时路径只有外部的,而且相对来说固定
# 而且只需要双击exe即可运行
# 我现在需要优化一下这个代码,使构建的 exe运行更流畅,体积更小,更少出问题, 更有普适性
# 然后我需要构建exe的命令行,我在windows平台,这是需要注意的.
# 我需要关键优化点做详细的注释

# 构建
# pip install pyinstaller
# pyinstaller --onefile --windowed delete_empty_folder
# --onefile：将所有依赖打包成一个单独的 .exe 文件
# --windowed：避免在运行时显示命令行窗口（适合双击运行）
# --clean：清理临时文件和缓存，确保构建过程更干净
# --noconsole：不显示控制台窗口（适用于GUI程序或不需要用户交互的脚本）
# --icon="path_to_icon.ico" 添加一个图标
# [UPX](https://upx.github.io/) --upx-dir=C:\path\to\upx 压缩体积
# 最终
# pyinstaller --onefile --upx-dir=D:\light\upx-4.2.4-win64 kill_baidu.py