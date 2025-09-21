// ?获取参数
//标准库中提供了函数 std::env::args()，它会返回一个迭代器，包含传入的参数。
//第一个参数（索引 0）通常是程序名（例如 grrs），其后的才是用户真正输入的内容。
//以下代码展示了最简单的获取方式（文件 src/main.rs）：

use std::{collections::{hash_map, HashMap}, fmt::format, fs::{self, File}, io::{self, stdin, BufRead, BufReader, Write}, process::Command, thread};

mod structs;


use clap::Parser;
use anyhow::{Context, Result};

use log::{info, warn};
use env_logger::Builder;

use structs::cli::Cli;



fn build_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("使用", "use");
    m.insert("公开", "pub");
    m.insert("函数", "fn");
    m.insert("主程序", "main");
    m.insert("绑定", "let");
    m.insert("返回", "return");
    m.insert("如果", "if");
    m.insert("否则", "else");
    m.insert("循环", "loop");
    m.insert("当", "while");
    m.insert("让", "let");
    m.insert("可变", "mut");
    m.insert("为", "for");
    m.insert("打印!", "print!");
    m.insert("打印ln!", "println!"); // 宏别名（注意！）
    m.insert("读取输入", "read_line");
    m.insert("引用", "&");
    m.insert("新建", "new");
    m.insert("字符串", "String");
    m.insert("转数字", "parse");
    m.insert("大于", ">");
    m.insert("小于", "<");
    m.insert("等于", "==");
    m.insert("预期", "expect");
    m.insert("标准输入", "io::stdin");
    m.insert("之", ".");
    m.insert("32位整数", "i32");
    m.insert("修剪", "trim");
    m
}


fn main()->Result<()> {
    let args = Cli::parse();

    // env_logger::init(); //初始化日志系统
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    
    info!("Rust中文编程");
    
    
    
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    let map = build_map();
    for (k, v) in &map {
        info!("{} -> {}",k,v);
    }
    
    // 读取文件内容,第二个参数
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("无法读取文件内容 `{:?}`", &args.path))?;

    println!("替换前");
    for line in  content.lines() {
       writeln!(handle, "{}", line)?;
    }

    handle.flush()?;
    
    println!("替换后");
    let mut replaced = content.clone();
    for (k, v) in map {
        replaced = replaced.replace(k, v);
    }
    for line in replaced.lines() {
        writeln!(handle, "{}", line)?;
    }
    handle.flush()?;

    
     // 写入临时文件
    let temp_file = "temp_output.rs";
    fs::write(temp_file, &replaced)?;

    // 调用 rustc 编译
    // let output = Command::new("rustc")
    //     .arg(temp_file)
    //     .arg("-o")
    //     .arg("猜数字游戏")
    //     .output()
    //     .expect("failed to execute rustc");
    let mut child = Command::new("rust-script")
        .arg(temp_file)
        .spawn()
        .expect("启动失败");
    child.wait().expect("等待shell失败");
    Ok(())
}


