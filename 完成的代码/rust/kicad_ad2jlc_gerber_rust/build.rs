extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("ferris/rustacean-flat-happy.ico");  // 设置图标文件路径
        res.compile().unwrap();
    }
}

