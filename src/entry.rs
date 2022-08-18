use chrono::prelude::*;
use std::convert::TryFrom;
use std::fmt::format;
use utmp::{utmp::Utmp, ulity};
use thiserror::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tabled::{Tabled, Table};
use base62;

#[derive(Tabled)]
#[derive(Clone, Default, Debug)]
pub struct UtmpEntry {
    #[tabled(rename = "Type Id", skip)]
    typeid: i32,
    #[tabled(rename = "UnionCode")]
    pub unioncode: String,
    #[tabled(rename = "Type")]
    typestr: String,
    #[tabled(display_with = "display_option")]
    #[tabled(rename = "Pid")]
    pub pid: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[tabled(rename = "Terminal")]
    line: Option<String>,
    #[tabled(display_with = "display_option", skip)]
    #[tabled(rename = "Terminal indentifier")]
    terminalid: Option<String>,
    #[tabled(display_with = "display_option")]
    #[tabled(rename = "Username")]
    username: Option<String>,
    #[tabled(display_with = "display_option")]
    #[tabled(rename = "Hostname")]
    pub hostname: Option<String>,
    #[tabled(display_with = "display_option", skip)]
    terminationstatus: Option<i16>,
    #[tabled(display_with = "display_option", skip)]
    exitstatus: Option<i16>,
    #[tabled(display_with = "display_option")]
    #[tabled(rename = "Session Id")]
    sessionid: Option<i32>,
    #[tabled(display_with = "display_option")]
    #[tabled(rename = "Time")]
    pub time: Option<NaiveDateTime>,
    #[tabled(display_with = "display_option")]
    #[tabled(rename = "IP Addr")]
    ipaddr: Option<IpAddr>,
}

fn display_option<T>(o: &Option<T>) -> String
    where T: std::fmt::Display
{
    match o {
        Some(s) => format!("{}", s.to_string()),
        None => format!(""),
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum UtmpError {
    #[error("Unknown Error: {0}")]
    ErrUnknown(String),

    #[error("Error happened during parse entry: {0:?}")]
    ErrDuringEntryParse(String),
}

// impl<'a> TryFrom<&'a Utmp> for UtmpEntry {
impl TryFrom<Utmp> for UtmpEntry {
    type Error = UtmpError;

    fn try_from(from: Utmp) -> Result<Self, UtmpError> {
        const ENTRY_TYPES: [&str; 12] = [
            "EMPTY",
            "RUN_LVL",
            "BOOT_TIME",
            "NEW_TIME",
            "OLD_TIME",
            "INIT_PROCESS",
            "LOGIN_PROCESS",
            "USER_PROCESS",
            "DEAD_PROCESS",
            "ACCOUNTING",
            "SIGNATURE",
            "SHUTDOWN_TIME"
        ];
        let mut tmpentry: UtmpEntry = UtmpEntry{
            typeid: 0,
            unioncode: "".to_string(),
            typestr: "".to_string(),
            pid: None,
            line: None,
            terminalid: None,
            username: None,
            hostname: None,
            terminationstatus: None,
            exitstatus: None,
            sessionid: None,
            time: None,
            ipaddr: None
        };
        match from.ut_type {
            0 => {
                tmpentry.typeid = 0;
                // tmpentry.typestr = ENTRY_TYPES[i as usize].to_string();
                tmpentry.typestr = format!("{:2} - {}",0,ENTRY_TYPES[0usize]);
                return Ok(tmpentry);
            },
            i @ 1..=11 => {
                tmpentry.typeid = i;
                // tmpentry.typestr = ENTRY_TYPES[i as usize].to_string();
                tmpentry.typestr = format!("{:2} - {}",i,ENTRY_TYPES[i as usize]);
            },
            ii => {
                return  Err(UtmpError::ErrDuringEntryParse(format!("ut_type({}) is invalid.",ii,)))
            }
        }
        tmpentry.unioncode = base62::encode(u64::from(from.ut_time_sec) * u64::from(u32::MAX) + from.ut_time_usec as u64);
        tmpentry.pid = Some(from.ut_pid);
        tmpentry.line = Some(ulity::extract_string(&from.ut_line));
        tmpentry.terminalid = Some(ulity::extract_string(&from.ut_id));
        tmpentry.username = Some(ulity::extract_string(&from.ut_user));
        tmpentry.hostname = Some(ulity::extract_string(&from.ut_host));
        tmpentry.terminationstatus = Some(from.ut_termination);
        tmpentry.exitstatus = Some(from.ut_exit);
        tmpentry.sessionid = Some(from.ut_session);
        tmpentry.time = Some(NaiveDateTime::from_timestamp(from.ut_time_sec as i64, from.ut_time_usec));
        let tmpipv6u8: [u8;16] = from.ut_addr_v6.iter()
            .map(|a|a.to_ne_bytes())
            .flatten()
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        tmpentry.ipaddr = {
            if from.ut_addr_v6[1..4] != [0,0,0] {
                Some(IpAddr::V6(Ipv6Addr::from(tmpipv6u8)))
            } else if from.ut_addr_v6[0] != 0 {
                Some(IpAddr::V4(Ipv4Addr::from(from.ut_addr_v6[0])))
            }
            else {
                None
            }};
        Ok(tmpentry)
    }
}

#[test]
fn test_base62() {
    // let (ut_time_sec, ut_time_usec) = ()


}