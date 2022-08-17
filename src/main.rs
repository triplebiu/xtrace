mod entry;

use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use tracing_subscriber::{prelude::*, layer::SubscriberExt, util::SubscriberInitExt};
use clap::{Args, Parser, Subcommand};
use utmp;
use utmp::UT_RECORDSIZE;
use crate::entry::UtmpEntry;
use tabled::{Tabled, Table, Style, Modify, Padding, object::Rows, Alignment};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Specify the target file.
    #[clap(short = 't', value_parser, value_name = "file",
    next_display_order = 1,
    default_values_os_t = vec ! [
    PathBuf::from("/run/utmp"),
    PathBuf::from("/var/log/wtmp"),
    PathBuf::from("/var/log/btmp"),
    PathBuf::from("files4test/utmp"),
    PathBuf::from("files4test/wtmp"),
    PathBuf::from("files4test/btmp"),
    ]
    )]
    targetfile: Vec<PathBuf>,

    /// search the condition to filter the records.
    ///
    /// UnionCode is the base62 of timestamps, it can consider as unique.
    #[clap(short = 's', value_name = "pid | Hostname | UnionCode")]
    condition: Option<Vec<String>>,

    /// Specify the record count to aim in the target file.
    #[clap(short = 'c', value_name = "number", default_value_t = 5, value_parser = clap::value_parser ! (u32).range(1..))]
    count: u32,

    /// DELETE the matched records in the target file(s).
    ///
    /// if the flag is false, it will just print out the records match the conditions.
    #[clap(short = 'D', action)]
    delete: bool,
}


fn main() {
    tracing_subscriber::registry()
        // .with(tracing_subscriber::EnvFilter::new(
        //     std::env::var("RUST_LOG").unwrap_or_else(|_| "XTRACE_LOG=debug".into()),
        // ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::debug!("Command args: {:?}",std::env::args_os());
    let cli: Cli = Cli::parse();
    tracing::debug!("Parsed command args: {:#?}",cli);

    // check target files.
    let existsfile = cli.targetfile.into_iter().filter(|f| f.exists()).collect::<Vec<_>>();

    // tracing::debug!("current dir: {:?}",std::env::current_dir());
    // tracing::debug!("testing fils exists:::: {:?}", PathBuf::from("../files4test/utmp").exists());

    if existsfile.is_empty() {
        tracing::error!("The target file(s) no exists.   Quiting!");
        return;
    }
    tracing::info!("exists files: {:?}",existsfile);

    // 遍历目标文件
    for target_file in existsfile {
        println!("\n[ Targeting on {} ]", target_file.to_string_lossy());

        let target_file_lenght = fs::metadata(&target_file).unwrap().len();

        if target_file_lenght > 384 * 5000 {
            tracing::warn!("Caution!!! The target file is too large. ({} bytes)", target_file_lenght);
        } else if target_file_lenght > 384 * 500 {
            tracing::warn!("Caution! The target file is a bit large. ({} bytes)", target_file_lenght);
        }
        if target_file_lenght % (UT_RECORDSIZE as u64) > 0 {
            println!("This file may not be a valid utmp file due to inappropriate file size.");
        } else {
            println!("Estimated amount of records in the file (by file size): {:5}", target_file_lenght / (UT_RECORDSIZE as u64));
        }

        let f = File::open(&target_file).unwrap();
        let mut reader = BufReader::new(f);
        let mut utmp_data = Vec::new();
        reader.read_to_end(&mut utmp_data).unwrap();

        // println!("utmp_data: {:?}",utmp_data);
        let (_, utmp_items) = utmp::take_all_records(&utmp_data).ok().unwrap();

        let mut utmpentries: Vec<UtmpEntry> = Vec::new();
        let mut save_back_data: Vec<u8> = Vec::new();
        for (_index, utmp_item) in utmp_items.iter().enumerate() {
            match UtmpEntry::try_from(utmp_item) {
                Ok(utmp_entry) => {
                    match &cli.condition {
                        Some(condition_vec) => if utmpentries.len() < cli.count as usize
                            && (((&utmp_entry).pid != None && condition_vec.contains(&utmp_entry.pid.unwrap_or(0).to_string()))
                            || ((&utmp_entry).hostname != None && condition_vec.contains(&utmp_entry.hostname.clone().unwrap_or(String::new())))
                            || ((&utmp_entry).unioncode.len() > 0 && condition_vec.contains(&utmp_entry.unioncode))
                        ) {
                            // match the conditions
                            assert!(utmpentries.len() < cli.count as usize);
                            // tracing::debug!("current utmpentries lenght: {}\tcurrent record: {}", utmpentries.len(), (&utmp_entry).unioncode);
                            utmpentries.push(utmp_entry);
                        } else if cli.delete {
                            // 暂存数据，用于覆盖源文件。
                            save_back_data.append(&mut utmp_item.as_bytes_vec());
                        } else {
                            continue;
                        },
                        None => {
                            if utmpentries.len() < cli.count as usize {
                                utmpentries.push(utmp_entry);
                            } else if cli.delete {
                                save_back_data.append(&mut utmp_item.as_bytes_vec());
                            } else {
                                continue;
                            }
                        }
                        _ => {}
                    };
                }
                Err(e) => {
                    tracing::error!("it sames not well-format utmp data.");
                    break;
                }
            }
        }

        if utmpentries.len() > 0 {
            println!("\n{}\n", Table::builder(utmpentries)
                .index()
                // .set_name(Some("INDEX".to_string()))
                .build()
                // .with(Modify::new(Rows::single(0)).with(Padding::new(0, 0, 1, 1).set_fill('>', '<', '^', 'V')))
                .with(Modify::new(Rows::single(0)).with(Alignment::center()))
                .with(Style::markdown())
                .to_string()
            );
            if cli.delete {
                print!("Are you sure to remove above entries from the file? (Yes/No) ");
                io::stdout().flush().unwrap();
                // println!("Original file size: {} bytes.\tNew file size: {} bytes.", utmp_data.len(), save_back_data.len());
                let stdin = io::stdin();
                for line in stdin.lock().lines() {
                    match line {
                        Err(_) => break,    // with ^Z
                        Ok(s) => if s.to_lowercase().trim_end().eq("yes") || s.to_lowercase().trim_end().eq("y") {
                            println!("override the original file.....");
                            // do override.....
                            // match write_to_file(target_file, save_back_data) {
                            match fs::write(target_file, save_back_data) {
                                Ok(_) => println!("Override Successful."),
                                Err(e) => tracing::error!("Something Wrong. | {}",e.to_string()),
                            }
                            break;
                        } else if s.to_lowercase().trim_end().eq("no") || s.to_lowercase().trim_end().eq("n") {
                            break;
                        } else {
                            print!("Are you sure to remove above entries from the file? (Yes/No) ");
                            io::stdout().flush().unwrap();
                        }
                    }
                }
            }
        } else {
            println!("---------  NO MATCHED RECORDS FOUND  ----------");
        }
    }
}

// fn write_to_file<T>(target: T, data: Vec<u8>) -> io::Result<()>
//     where T: AsRef<Path>
// {
//     fs::write(target, data)?;
//     Ok(())
// }

#[test]
fn 测试驱动开发() {
    println!("一个比较好的命令行应用开发指引：{}", "https://suibianxiedianer.github.io/rust-cli-book-zh_CN/README_zh.html");
    assert_eq!(42, 42);
}

#[test]
fn verify_cli() {
    let cli: Cli = Cli::parse_from(["xtrace", "-t/run/utmp", "-t/var/log/wtmp", "-D", "-c10", "-s127.0.0.1", "-s9527"].into_iter());
    println!("cli: {:#?}", cli);
    assert_eq!(cli.delete, true);
    assert_eq!(cli.count, 10);
    assert_eq!(cli.condition, Some(vec!["127.0.0.1".to_string(), "9527".to_string()]));
}


#[test]
fn list_utmp_entries() {
    const UTMPDATA: &[u8] = include_bytes!("../files4test/utmp");

    let (_, res2) = utmp::take_all_records(UTMPDATA).ok().unwrap();

    let mut utmpentries: Vec<UtmpEntry> = Vec::new();
    for (_index, utmp_item) in res2.iter().enumerate() {
        // println!("ut_type: {:}\tut_host: {:40}\tut_time_sec: {}",
        //          utmp_item.ut_type,
        //          utmp::ulity::extract_string(&utmp_item.ut_host),
        //          utmp_item.ut_time_sec);
        utmpentries.push(utmp_item.try_into().unwrap());
        // println!("{:?}", utmp_item);
    }
    // println!("{}",Table::new(utmpentries).to_string());
    println!("{}", Table::builder(utmpentries)
        .index()
        // .set_name(Some("INDEX".to_string()))
        .build()
        // .with(Modify::new(Rows::single(0)).with(Padding::new(0, 0, 1, 1).set_fill('>', '<', '^', 'V')))
        .with(Modify::new(Rows::single(0)).with(Alignment::center()))
        .with(Style::markdown())
        .to_string()
    );
}

#[test]
fn check_files() {
    let target_file = "files4test/utmp".to_string();
    let target_file_lenght = fs::metadata(&target_file).unwrap().len();
    println!("{} len: {}", target_file, target_file_lenght);

    let f = File::open(target_file).unwrap();
    let mut reader = BufReader::new(f);
    let mut utmp_data = Vec::new();
    reader.read_to_end(&mut utmp_data).unwrap();

    println!("utmp_data: {:?}", utmp_data);
    let (_, res2) = utmp::take_all_records(&utmp_data).ok().unwrap();
    eprintln!("record amount: {}", res2.len())
}