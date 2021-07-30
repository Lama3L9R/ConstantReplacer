extern crate walkdir;

use std::env::args;
use std::fs;
use std::path::Path;

use walkdir::WalkDir;
use std::io::{Read, Write};
use std::process::exit;

fn main() {
    let arguments: Vec<String> = args().into_iter().map(|value| { value }).collect();

    let quite_flag = arguments.contains(&"-q".to_string());

    if arguments.contains(&"-l".to_string()) {
        arguments.iter().for_each(|arg| { println!("参数: {}", arg); });
    }

    if arguments.contains(&"-v".to_string()) {
        println!("ConstantReplacer ver 1.0 作者: Lama3L9R 语言: Rust 支持库: walkdir Github: Lama3L9R/ConstantReplacer 协议: Anti-996");
        return;
    }

    if arguments.contains(&"-h".to_string()) {
        println!("需要三个参数: ");
        println!(" 1. 需要被替换的文本");
        println!(" 2. 将要替换的文本");
        println!(" 3. 目标根目录(自动递归子目录)");
        println!(" [可选] -q 安静模式, 不输出任何东西");
        println!(" [可选] -h 显示这个帮助信息, 不执行任何操作");
        println!(" [可选] -v 显示版本信息, 不执行任何操作");
        return;
    }

    if args().len() >= 4 {
        if fs::read_dir(&arguments[3]).is_ok() {
            if !quite_flag {
                println!("将替换 {} 目录下所有文件从 {} 到 {} ", arguments[3], arguments[1], arguments[2]);
            }
            for file_entry in WalkDir::new(Path::new(&arguments[3])) {
                if file_entry.is_ok() {
                    let file = file_entry.unwrap();
                    if !file.file_type().is_dir() {
                        let try_source_file = fs::OpenOptions::new().read(true).open(file.path());
                        if try_source_file.is_err() {
                            if !quite_flag {
                                println!("文件 {} 无权访问", file.file_name().to_str().unwrap());
                            }
                            continue;
                        }

                        let mut source_file = try_source_file.unwrap();
                        let mut contents = String::new();
                        if source_file.read_to_string(&mut contents).is_err() {
                            if !quite_flag {
                                println!("文件 {} 无权访问", file.file_name().to_str().unwrap());
                            }
                            continue;
                        }

                        if !contents.contains(&arguments[1]) {
                            continue;
                        }

                        let changed = contents.replace(&arguments[1], &arguments[2]);

                        drop(source_file);

                        let try_source_file = fs::OpenOptions::new().read(true).write(true).append(false).truncate(true).open(file.path());
                        let mut source_file = try_source_file.unwrap();

                        if source_file.write_all(changed.as_bytes()).is_err() {
                            if !quite_flag {
                                println!("文件 {} 无权访问", file.file_name().to_str().unwrap());
                            }
                            continue;
                        }

                        if source_file.flush().is_err() {
                            if !quite_flag {
                                println!("文件 {} 无权访问", file.file_name().to_str().unwrap());
                            }
                            continue;
                        }
                    }
                } else {
                    if !quite_flag {
                        println!("忽略文件 null 无权访问!");
                    }
                }
            }
        } else {
            if !quite_flag {
                println!("{} 不存在或无权访问", &arguments[3]);
                exit(1);
            }
        }
    } else {
        if !quite_flag {
            println!("参数不正确! {} 需要被替换的字符串 需要替换的字符串 根目录", arguments[0]);
            exit(1);
        }
    }
}
