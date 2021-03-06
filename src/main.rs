extern crate clap;

use clap::{Arg, App};
use std::path::Path;
use std::process;
use std::fs::File;
use std::io::{Read, Write};

/**
 * https://juejin.cn/post/6844903821307723789
 */

fn main() {
    let matches = App::new("skd")
      .version("0.0.1")
      .author("kinsonwu. kingson4wu@gmail.com")
      .about("A unknown tool written in Rust")
      .arg(Arg::with_name("FILE")
            .help("File to print.")
            .empty_values(false)
        )
      .get_matches();

     if let Some(file) = matches.value_of("FILE") {
        if Path::new(&file).exists() {
            match File::open(file) {
                Ok(mut f) => {
                    let mut data = String::new();
                    f.read_to_string(&mut data).expect("[kt Error] Unable to read the  file.");
                    let stdout = std::io::stdout(); // 获取全局 stdout 对象
                    let mut handle = std::io::BufWriter::new(stdout); // 可选项：将 handle 包装在缓冲区中
                    match writeln!(handle, "{}", data) {
                        Ok(_res) => {},
                        Err(err) => {
                            eprintln!("[kt Error] Unable to display the file contents. {:?}", err);
                            process::exit(1);
                        },
                    }
                }
                Err(err) => {
                    eprintln!("[skd Error] Unable to read the file. {:?}", err);
                    process::exit(1);
                },
            }
        }
        else {
            eprintln!("[skd Error] No such file or directory.");
            process::exit(1);
        }
    }



}
