use nom::IResult;
use nom::error::Error;
use nom::bytes::complete::{take};
use nom::combinator::map;
use nom::multi::{count, many0};
use nom::number::complete::i32;
use crate::utmp::Utmp;

mod ulity;
mod utmp;


/// Record does not contain valid info (formerly known as `UT_UNKNOWN` on Linux)
pub const EMPTY: i32 = 0;
pub const UT_UNKNOWN: i32 = 0;
/// Change in system run-level (see `init(8)`)
pub const RUN_LVL: i32 = 1;
/// Time of system boot (in `ut_tv`)
pub const BOOT_TIME: i32 = 2;
/// Time after system clock change (in `ut_tv`)
pub const NEW_TIME: i32 = 3;
/// Time before system clock change (in `ut_tv`)
pub const OLD_TIME: i32 = 4;
/// Process spawned by `init(8)`
pub const INIT_PROCESS: i32 = 5;
/// Session leader process for user login
pub const LOGIN_PROCESS: i32 = 6;
/// Normal process
pub const USER_PROCESS: i32 = 7;
/// Terminated process
pub const DEAD_PROCESS: i32 = 8;
/// Not implemented
pub const ACCOUNTING: i32 = 9;
/// Signature, value used in file header
pub const SIGNATURE: i32 = 10;
/// Time of system shutdown
pub const SHUTDOWN_TIME: i32 = 11;

pub const UT_LINESIZE: usize = 32;
pub const UT_NAMESIZE: usize = 32;
pub const UT_HOSTSIZE: usize = 256;

fn take_all_records(i: &[u8]) -> IResult<&[u8], Vec<Utmp>> {
    many0(take_one_record)(i)
}

fn take_one_record(i: &[u8]) -> IResult<&[u8], Utmp> {
    let (i,ut_type) = take(4u32)(i)?;
    let (i,ut_pid) = take(4u32)(i)?;
    let (i,ut_line) = map(take(32u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_id) = map(take(4u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_user) = map(take(32u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_host) = map(take(256u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_termination) = map(take(2u32), |s: &[u8]| i16::from_ne_bytes(s.try_into().unwrap_or([0u8;2])))(i)?;
    let (i,ut_exit) = map(take(2u32), |s: &[u8]| i16::from_ne_bytes(s.try_into().unwrap_or([0u8;2])))(i)?;
    // let (i,ut_session) = map(take(4u32), |s: &[u8]| i32::from_ne_bytes(s.try_into().unwrap_or([0u8;4])))(i)?;
    let (i,ut_session) = i32(nom::number::Endianness::Native)(i)?;
    let (i,ut_time_sec) = map(take(4u32), |s: &[u8]| u32::from_ne_bytes(s.try_into().unwrap_or([0u8;4])))(i)?;
    let (i,ut_time_usec) = map(take(4u32), |s: &[u8]| u32::from_ne_bytes(s.try_into().unwrap_or([0u8;4])))(i)?;
    // let (i,ut_addr_v6) = count(take(4u32),4)(i)?;
    let (i,ut_addr_v6) = count(map(take(4u32), |s: &[u8]| u32::from_ne_bytes(s.try_into().unwrap_or([0u8;4]))),4)(i)?;
    let (i,__unused) = map(take(20u32),|s: &[u8]| ulity::extract_string(s))(i)?;

    Ok((i, Utmp{
        ut_type: i32::from_ne_bytes(ut_type.try_into().unwrap_or([0u8;4])),
        // ut_type: i32::from_ne_bytes([2, 0, 0, 0]),
        ut_pid: i32::from_ne_bytes(ut_pid.try_into().unwrap_or([0u8;4])),     // todo 返回引用数组，如何将&[u8]转换为i32 ?
        // ut_line: [0; 32],
        ut_line: ut_line,
        ut_id: ut_id,
        ut_user: ut_user,
        ut_host: ut_host,
        ut_termination: ut_termination,
        ut_exit: ut_exit,
        ut_session: ut_session,
        ut_time_sec: ut_time_sec,
        ut_time_usec: ut_time_usec,
        ut_addr_v4: ut_addr_v6[0].clone(),
        ut_addr_v6: ut_addr_v6.try_into().unwrap(),
        __unused: __unused,
    }))
}


fn main() {}


#[cfg(test)]
mod tests {
    use nom::error::Error;
    use nom::bytes::complete::take;
    // use crate::{ulity, Color};
    use super::*;

    const utmpdata: &[u8] = include_bytes!("../../files4test/utmp");

    #[test]
    /// test the utmp file: SHA256(../files4test/utmp)= 763b68385d4255c862e9024d021545dec07135b7170f1160f7e00fe3cd5cee2f
    fn test_utmp_parser() {
        // let res = take::<_, _, Error<_>>(384u32)(utmpdata);
        let (_,res) = take_one_record(utmpdata).ok().unwrap();
        // println!("{:#?}",res);
        assert_eq!(res.ut_host,"5.4.17-2102.203.6.el8uek.x86_64");

        let (_, res2) = take_all_records(utmpdata).ok().unwrap();
        assert_eq!(res2.len(),6);
        println!("\n{:?}",res2);
        assert_eq!(res2[5].ut_addr_v4, 3649615982);
    }

    #[test]
    fn test_ulity_hex2bytes() {
        let res = ulity::hex_to_bytes("0x3230332e362e656c3875656b2e783836");
        println!("{:?}", res);
        assert_eq!(res, Some(vec![50, 48, 51, 46, 54, 46, 101, 108, 56, 117, 101, 107, 46, 120, 56, 54]));
    }
}