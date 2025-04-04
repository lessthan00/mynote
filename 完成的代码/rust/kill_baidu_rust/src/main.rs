use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::{
    UI::Shell::{SHGetFolderPathW, CSIDL_APPDATA},
};

/// 安全获取 AppData 目录路径，使用 SHGetFolderPathW 替代 SHGetKnownFolderPath
fn get_appdata_path() -> Option<PathBuf> {
    let mut path_buffer: [u16; 260] = [0; 260]; // MAX_PATH 长度
    unsafe {
        // 将CSIDL_APPDATA转换为i32类型以匹配函数签名
        if SHGetFolderPathW(None, CSIDL_APPDATA as i32, None, 0, &mut path_buffer).is_ok() {
            let len = path_buffer.iter().position(|&c| c == 0).unwrap_or(path_buffer.len());
            let slice = &path_buffer[..len];
            let os_string = OsString::from_wide(slice);
            Some(PathBuf::from(os_string))
        } else {
            None
        }
    }
}

/// 安全删除文件夹，处理权限问题和路径不存在的情况
fn remove_folders(folders: &[&str], base_path: &PathBuf) {
    for folder in folders {
        let path = base_path.join(folder);
        match fs::remove_dir_all(&path) {
            Ok(_) => println!("成功删除: {:?}", path),
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => println!("路径不存在: {:?}", path),
                io::ErrorKind::PermissionDenied => println!("权限不足: {:?}", path),
                _ => println!("删除 {:?} 失败: {}", path, e),
            },
        }
    }
}

fn main() {
    let folders = ["BaiduYunGuanjia", "BaiduYunKernel", "Baidu"];

    let appdata_path = get_appdata_path().unwrap_or_else(|| {
        println!("错误：无法获取AppData路径");
        std::process::exit(1);
    });

    if !appdata_path.exists() || !appdata_path.is_dir() {
        println!("错误：AppData路径无效");
        std::process::exit(1);
    }

    remove_folders(&folders, &appdata_path);

    print!("操作完成，按回车键退出...");
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

// 需要写一个rust程序, cargo new kill_baidu_rust,编译成windows11 x64 exe可执行程序,
// 双击时删除 %AppData%\Roaming 里面的百度网盘文件，也就是["BaiduYunGuanjia", "BaiduYunKernel", "Baidu"]代表的文件夹和其内容。

// 引用其他的包或API时，首先需要考虑其稳定性和是否被淘汰，太新而不稳定，太旧而被淘汰的都不用，考虑后，再写cargo.toml和use 包。
// 写代码需要优先考虑稳定，安全，可以验证可能发生的报错，需要有错误处理
// 写代码需要优先保证exe能够运行顺畅,稳定,具有普适性.
// 写代码需要写好中文注释,尤其是保证稳定性和安全行的代码
// 需要给我完整的main.rs的代码和 cargo.toml的代码

// 报错如下，返回修改和其上下文，通常是一整个函数
