use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_INPUT_HANDLE;
use winapi::um::winuser::{MessageBoxA, MB_OK};
use std::ffi::CString;

fn main() {
    // 获取当前工作目录
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // 调用函数删除空文件夹
    if let Err(e) = remove_empty_folders(&current_dir) {
        eprintln!("Error: {}", e);
    }

    // 在 Windows 下，防止窗口立即关闭
    if cfg!(windows) {
        wait_for_enter();
    }
}

/// 删除指定路径下的所有空文件夹
fn remove_empty_folders(path: &Path) -> io::Result<()> {
    // 使用 WalkDir 遍历文件夹，contents_first=true 表示从子文件夹开始遍历（确保先删除子文件夹）
    for entry in WalkDir::new(path).min_depth(1).contents_first(true) {
        let entry = entry?;
        let dir_path = entry.path();

        // 检查是否为文件夹
        if dir_path.is_dir() {
            // 检查文件夹是否为空
            if is_empty(dir_path)? {
                // 删除空文件夹
                fs::remove_dir(dir_path)?;
                println!("Deleted empty folder: {:?}", dir_path);
            }
        }
    }
    Ok(())
}

/// 检查文件夹是否为空
fn is_empty(path: &Path) -> io::Result<bool> {
    Ok(path.read_dir()?.next().is_none())
}

/// 在 Windows 下等待用户按下 Enter 键
fn wait_for_enter() {
    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        if handle.is_null() {
            // 如果获取标准输入句柄失败，显示消息框
            let msg = CString::new("Press Enter to exit...").unwrap();
            MessageBoxA(std::ptr::null_mut(), msg.as_ptr(), msg.as_ptr(), MB_OK);
        } else {
            // 否则，等待用户输入
            println!("Press Enter to exit...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
    }
}


// 这是一个rust并构建为windows x64 exe运行程序,在Windows平台运行,具有普适性和流畅性,
// 已经 cargo new delete_empty_folder,需要main.rs和 cargo.toml的代码
// 代码需要详细的注释
// 代码需要实现双击运行构建的exe,然后其所在父目录的所有空的子目录递归被删除.
// 我需要的是后面构建的exe双击运行,所以需要保障exe运行的顺利
// 需要构建的exe的体积小,运行稳定
// 需要exe加上 rust的吉祥物图.
// 需要给我构建的指令

// 构建
// cargo build --release
// 构建完成后，EXE 文件位于 target/release/delete_empty_folder.exe