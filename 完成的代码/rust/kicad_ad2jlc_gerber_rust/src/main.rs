use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::env; // 导入 env 模块
use std::io;

use csv::ReaderBuilder;
use serde::Deserialize;
use zip::write::{FileOptions, ZipWriter};

// Import needed chrono parts
use chrono::Local;

// Import walkdir
use walkdir::WalkDir;

extern crate lazy_static;
use lazy_static::lazy_static;

lazy_static! {
    static ref BASE_PATH: PathBuf = {
        env::current_dir().unwrap_or_else(|_| {
            env::current_exe()
                .map(|p| p.parent().unwrap().to_path_buf())
                .unwrap()
        })
    };

    static ref GERBER_CSV: PathBuf = BASE_PATH.join("data/gerber.csv");
    static ref GERBER_CSV_CONTENT: &'static str = include_str!("../data/gerber.csv");
    static ref HEADER_FILE_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("hear1", include_str!("../data/jlc_gerber_header.md"));
        m.insert("hear2", include_str!("../data/jlc_gerber_header2.md"));
        m
    };

    static ref PATH_FINAL: PathBuf = BASE_PATH.join("jlc_gerber");
    static ref AD_GERBER_FOLDER: PathBuf = BASE_PATH.join("ad_gerber");
    static ref KICAD_GERBER_FOLDER: PathBuf = BASE_PATH.join("kicad_gerber");
    
    static ref ZIP_PATH: PathBuf = {
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let project_name = BASE_PATH
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        BASE_PATH.join(format!("out_{}-{}.zip", project_name, timestamp))
    };
}


#[derive(Debug, Deserialize)]
struct CsvRow {
    ad_gbr: String,
    ad_filename: String,
    kicad_gbr: String,
    kicad_filename: String,
    kicad_filename2: String,
    jlc_filename: String,
    jlc_begin: String,
    hear: String,
}

fn create_folder(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(path)?;
    println!("Folder '{:?}' ready", path);
    Ok(())
}

fn clean_folder(path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let item_path = entry.path();

        if item_path.is_file() {
            fs::remove_file(&item_path)?;
        } else {
            fs::remove_dir_all(&item_path)?;
        }
    }
    println!("Folder '{:?}' cleaned", path);
    Ok(())
}

fn load_csv_data(file_path: &Path) -> Result<Vec<CsvRow>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().from_reader(reader);
    let mut data = Vec::new();

    for result in csv_reader.deserialize() {
        let record: CsvRow = result?;
        data.push(record);
    }

    Ok(data)
}

fn copy_and_rename_files(source_path: &Path, file_name: &str, file_new_name: &str) -> Result<(), Box<dyn Error>> {
    let source_file = source_path.join(file_name);
    let destination_file = PATH_FINAL.join(file_new_name);
    println!("copyfile: {:?}", file_name);
    fs::copy(source_file, destination_file)?;
    Ok(())
}

fn add_file_header(file_path: &Path, row: &CsvRow) -> Result<(), Box<dyn Error>> {
    let header_begin = row.jlc_begin.trim();
    let header_type = &row.hear;

    // 读取原有内容
    let mut original_content = String::new();
    let mut file = File::open(file_path)?;
    file.read_to_string(&mut original_content)?;

    // 构建新内容
    let mut new_content = Vec::new();
    new_content.push(header_begin.to_string());

    // 添加对应Header文件内容
    if let Some(header_file_content) = HEADER_FILE_MAP.get(header_type.as_str()) {
        new_content.push(header_file_content.trim().to_string());
    }

    new_content.push(original_content);

    // 写入文件
    let mut file = File::create(file_path)?;
    file.write_all(new_content.join("\n").as_bytes())?; // 用换行符连接所有部分
    println!("Header added to: {:?}", file_path);

    Ok(())
}

fn zip_folder(folder_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(folder_path)?;

        // Write file or directory explicitly
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        } else if name.as_os_str().len() != 0 {
            //  Only add the folder if not root
            println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }

    zip.finish()?;
    println!("ZIP created: {:?}", output_path);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {

    // 初始化目标文件夹
    create_folder(&PATH_FINAL)?;
    clean_folder(&PATH_FINAL)?;

    // 加载CSV配置（添加文件存在性检查）
    if !GERBER_CSV.exists() {
        return Err(format!("CSV文件不存在于: {}", GERBER_CSV.display()).into());
    }
    let csv_data = load_csv_data(&GERBER_CSV)?;

    // 动态选择要使用的文件夹
    let source_folder = if AD_GERBER_FOLDER.exists() {
        &*AD_GERBER_FOLDER
    } else if KICAD_GERBER_FOLDER.exists() {
        &*KICAD_GERBER_FOLDER
    } else {
        return Err("AD_GERBER_FOLDER 和 KICAD_GERBER_FOLDER 都不存在".into());
    };

    println!("Using source folder: {:?}", source_folder);

    // 遍历 CSV 数据并处理文件
    for row in &csv_data {
        for entry in fs::read_dir(source_folder)? {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            if !row.ad_gbr.is_empty() && file_name.contains(&row.ad_gbr) {
                copy_and_rename_files(source_folder, &file_name, &row.jlc_filename)?;
            }
            if !row.ad_filename.is_empty() && file_name.contains(&row.ad_filename) {
                copy_and_rename_files(source_folder, &file_name, &row.jlc_filename)?;
            }
            if !row.kicad_gbr.is_empty() && file_name.contains(&row.kicad_gbr) {
                copy_and_rename_files(source_folder, &file_name, &row.jlc_filename)?;
            }
            if !row.kicad_filename.is_empty() && file_name.contains(&row.kicad_filename) {
                copy_and_rename_files(source_folder, &file_name, &row.jlc_filename)?;
            }
            if !row.kicad_filename2.is_empty() && file_name.contains(&row.kicad_filename2) {
                copy_and_rename_files(source_folder, &file_name, &row.jlc_filename)?;
            }
        }
    }
    // 暂停一下
    println!("按回车键继续...");
    let _ = io::stdin().read_line(&mut String::new());

    // 添加文件头
    for row in &csv_data {
        let file_new_path = PATH_FINAL.join(&row.jlc_filename);
        if file_new_path.exists() {
            add_file_header(&file_new_path, row)?;
        }
    }

    // 打包文件
    zip_folder(&PATH_FINAL, &ZIP_PATH)?;

    println!("处理完成，ZIP 文件已生成: {:?}", ZIP_PATH.display());

    // 等待用户输入，防止控制台窗口关闭
    println!("按回车键退出...");
    let _ = io::stdin().read_line(&mut String::new());

    Ok(())
}