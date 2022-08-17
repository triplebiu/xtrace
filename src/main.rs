use std::path::{Path, PathBuf};
use tracing_subscriber::{prelude::*, layer::SubscriberExt, util::SubscriberInitExt};
use clap::{Args, Parser, Subcommand};
use utmp;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Specify the target file.
    #[clap(short='t', value_parser, value_name = "file",
        next_display_order = 1,
        default_values_os_t = vec![
            PathBuf::from("/run/utmp"),
            PathBuf::from("/var/log/wtmp"),
            PathBuf::from("/var/log/btmp"),
            PathBuf::from("../files4test/utmp"),
            PathBuf::from("files4test/wtmp"),
            PathBuf::from("files4test/btmp"),
        ]
    )]
    targetfile: Vec<PathBuf>,

    /// search the condition to filter the records.
    ///
    /// if does not specify the option, it will filter out no records.
    #[clap(short='s', value_name= "pid | IP")]
    condition: Option<Vec<String>>,

    /// Specify the record count to aim in the target file.
    #[clap(short='c', value_name= "number", default_value_t = 8)]
    count: u32,

    /// Delete matched records in the target file(s).
    ///
    /// if the flag is false, it will just print out the records match the conditions.
    #[clap(short='D', action)]
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
    let existsfile= cli.targetfile.into_iter().filter(|f| f.exists()).collect::<Vec<_>>();

    // tracing::debug!("current dir: {:?}",std::env::current_dir());
    // tracing::debug!("testing fils exists:::: {:?}", PathBuf::from("../files4test/utmp").exists());

    if existsfile.is_empty() {
        tracing::error!("The target file(s) no exists.   Quiting!");
        return;
    }
    tracing::info!("exists files: {:?}",existsfile);

    const UTMPDATA: &[u8] = include_bytes!("../files4test/utmp");
    if cli.delete == false {

        let (_, res2) = utmp::take_all_records(UTMPDATA).ok().unwrap();

        for (_index, utmp_item) in res2.iter().enumerate() {
            println!("ut_type: {:}\tut_host: {:40}\tut_time_sec: {}",
                     utmp_item.ut_type,
                     utmp::ulity::extract_string(&utmp_item.ut_host),
                     utmp_item.ut_time_sec);
        }
    }

}


#[test]
fn 测试驱动开发() {
    println!("一个比较好的命令行应用开发指引：{}","https://suibianxiedianer.github.io/rust-cli-book-zh_CN/README_zh.html");
    assert_eq!(42, 42);
}

#[test]
fn verify_cli() {
    let cli: Cli = Cli::parse_from(["xtrace", "-t/run/utmp", "-t/var/log/wtmp", "-D", "-c10", "-s127.0.0.1", "-s9527"].into_iter());
    println!("cli: {:#?}",cli);
    assert_eq!(cli.delete,true);
    assert_eq!(cli.count, 10);
    assert_eq!(cli.condition, Some(vec!["127.0.0.1".to_string(),"9527".to_string()]));
}

