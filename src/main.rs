mod entry;
use chrono::prelude::*;

use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::collections::VecDeque;
use tracing_subscriber::{prelude::*, layer::SubscriberExt, util::SubscriberInitExt};
use clap::{Args, Parser, Subcommand};
use tracing_subscriber::filter::LevelFilter;
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
    // PathBuf::from("files4test/utmp"),
    // PathBuf::from("files4test/wtmp"),
    // PathBuf::from("files4test/btmp"),
    ]
    )]
    targetfile: Vec<PathBuf>,

    /// search the condition to filter the records.
    ///
    /// UnionCode is the base62 of timestamps, it can consider as unique.
    #[clap(short = 's', value_name = "Pid | Hostname | UnionCode")]
    condition: Option<Vec<String>>,

    /// Specify the record count to aim in the target file.
    ///
    /// if set 0, it means all records.
    #[clap(short = 'c', value_name = "number", default_value_t = 5)]
    count: u32,

    /// DELETE the matched records in the target file(s).
    ///
    /// if the flag does set, it will just print out the records match the conditions.
    #[clap(short = 'D', action)]
    delete: bool,
}


fn main() {
    tracing_subscriber::registry()
        // .with(tracing_subscriber::EnvFilter::new(
        //     std::env::var("XTRACE_LOG").unwrap_or_else(|_| "XTRACE_LOG=warn".into()),
        // ))
        .with(tracing_subscriber::fmt::layer().with_filter(LevelFilter::WARN))
        .init();

    // tracing::error!("tracing::error test");
    // tracing::warn!("tracing::warn test");
    // tracing::info!("tracing::info test");
    // tracing::debug!("tracing::debug test");
    // tracing::trace!("tracing::trace test");
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
    println!("Target Files: {:?}\nFilter Conditions: {:?}\nMax Count: {}"
        ,existsfile
        ,&cli.condition.clone().unwrap_or(Vec::new())
        , if cli.count ==0 { "All".to_string()} else { cli.count.to_string()});

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
            println!("Caution! This file may not be a valid utmp file due to inappropriate file size.");
        } else {
            println!("Estimated amount of records in the file (by file size): {:5}\nThe Matched Records: ", target_file_lenght / (UT_RECORDSIZE as u64));
        }

        let f = File::open(&target_file).unwrap();
        let mut reader = BufReader::new(f);
        let mut utmp_data = Vec::new();
        reader.read_to_end(&mut utmp_data).unwrap();

        // println!("utmp_data: {:?}",utmp_data);
        // let (_, utmp_items) = utmp::take_all_records(&utmp_data).ok().unwrap();
        let (_, utmp_items_with_original_data) = utmp::take_all_records_with_original_data(&utmp_data).ok().unwrap();


        // let mut utmpentries_with_postion: Vec<(u32,UtmpEntry)> = Vec::new();
        let mut utmpentries_with_postion: VecDeque<(u32,UtmpEntry)> = VecDeque::new();
        let mut utmp_data_with_remove_marks: Vec<(bool, Vec<u8>)> = Vec::new();    // bool用于标注每个utmp数据段是否保留。
        for (index, (original_data, utmp_item)) in utmp_items_with_original_data.into_iter().enumerate() {

            match UtmpEntry::try_from(utmp_item) {
                Ok(utmp_entry) => {
                    match &cli.condition {
                        Some(condition_vec) => if ((&utmp_entry).pid != None && condition_vec.contains(&utmp_entry.pid.unwrap_or(0).to_string()))
                            || ((&utmp_entry).hostname != None && condition_vec.contains(&utmp_entry.hostname.clone().unwrap_or(String::new())))
                            || ((&utmp_entry).unioncode.len() > 0 && condition_vec.contains(&utmp_entry.unioncode))
                        {
                            // match the conditions
                            // utmpentries.push(utmp_entry);
                            utmp_data_with_remove_marks.push((false, original_data));
                            utmpentries_with_postion.push_front((index as u32, utmp_entry));

                            // 数据条目超出限制后，pop旧条目，并修改原始数据数组的标记。
                            if (utmpentries_with_postion.len() > cli.count as usize && cli.count != 0){
                                let (original_position, _) = utmpentries_with_postion.pop_back().unwrap();
                                utmp_data_with_remove_marks[original_position as usize] = (true, utmp_data_with_remove_marks[original_position as usize].1.clone());
                            }
                        } else{
                            // 对于不满足条件的条目，直接记录。
                            utmp_data_with_remove_marks.push((true, original_data));
                        },
                        None => {
                            if (utmpentries_with_postion.len() < cli.count as usize || cli.count == 0) {
                                utmp_data_with_remove_marks.push((false, original_data));
                                utmpentries_with_postion.push_front((index as u32, utmp_entry));
                            } else{
                                utmp_data_with_remove_marks.push((false, original_data));
                                utmpentries_with_postion.push_front((index as u32, utmp_entry));

                                let (original_position, _) = utmpentries_with_postion.pop_back().unwrap();
                                utmp_data_with_remove_marks[original_position as usize] = (true, utmp_data_with_remove_marks[original_position as usize].1.clone());

                            }
                        }
                        _ => {}
                    };
                    // println!("{:?}", utmpentries_with_postion.iter().map(|s|s.1.time).collect::<Vec<Option<NaiveDateTime>>>());
                }
                Err(e) => {
                    tracing::error!("it sames not well-format utmp data.");
                    break;
                }
            }
        }

        if utmpentries_with_postion.len() > 0 {
            // utmpentries_with_postion.reverse();
            println!("\n{}\n", Table::builder(utmpentries_with_postion.into_iter().map(|t|t.1).collect::<Vec<_>>())
                .index()
                // .set_name(Some("INDEX".to_string())).clone()
                .build()
                // .with(Modify::new(Rows::single(0)).with(Padding::new(0, 0, 1, 1).set_fill('>', '<', '^', 'V')))
                .with(Modify::new(Rows::single(0)).with(Alignment::center()))
                .with(Style::markdown())
                .to_string()
            );
            if cli.delete {
                // print!("Are you sure to remove above entries from the file? (Yes/No) ");
                // io::stdout().flush().unwrap();
                // println!("Original file size: {} bytes.\tNew file size: {} bytes.", utmp_data.len(), save_back_data.len());
                match fs::write(&target_file,
                                utmp_data_with_remove_marks.into_iter().
                                    map(|(mark, data)| if mark { data } else { Vec::new() }).flatten()
                                    .collect::<Vec<_>>(),
                ) {
                    Ok(_) => println!("Complete. The above records have been deleted."),
                    Err(e) => tracing::error!("Something Wrong. | {}",e.to_string()),
                }
                // let stdin = io::stdin();
                // for line in stdin.lock().lines() {
                //     match line {
                //         Err(_) => break,    // with ^Z
                //         Ok(s) => if s.to_lowercase().trim_end().eq("yes") || s.to_lowercase().trim_end().eq("y") {
                //             match fs::write(&target_file,
                //                             utmp_data_with_remove_marks.into_iter().
                //                                 map(|(mark, data)| if mark { data } else { Vec::new() }).flatten()
                //                                 .collect::<Vec<_>>(),
                //             ) {
                //                 Ok(_) => println!("Done."),
                //                 Err(e) => tracing::error!("Something Wrong. | {}",e.to_string()),
                //             }
                //             break;
                //         } else if s.to_lowercase().trim_end().eq("no") || s.to_lowercase().trim_end().eq("n") {
                //             break;
                //         } else {
                //             print!("Are you sure to remove above entries from the file? (Yes/No) ");
                //             io::stdout().flush().unwrap();
                //         }
                //     }
                // }
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