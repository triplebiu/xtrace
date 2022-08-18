use nom::IResult;
// use nom::error::Error;
use nom::bytes::complete::{take};
use nom::combinator::map;
use nom::multi::{count, many0, many_m_n};
use nom::number::complete::i32;
use crate::utmp::Utmp;


pub mod ulity;
pub mod utmp;


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
pub const UT_RECORDSIZE: usize = 384;

pub fn take_all_records(i: &[u8]) -> IResult<&[u8], Vec<Utmp>> {
    many0(take_one_record)(i)
}

pub fn take_all_records_with_original_data(i: &[u8]) -> IResult<&[u8], Vec<(Vec<u8>, Utmp)>> {
    many0(take_one_record_with_original_data)(i)
}

// pub fn take_n_records(i: &[u8], n: u32) -> IResult<&[u8], Vec<Utmp>> {
//     many_m_n(0,n as usize,take_one_record)(i)
// }

pub fn take_one_record_with_original_data(i: &[u8]) -> IResult<&[u8], (Vec<u8>, Utmp)> {
    let data:Vec<u8> = if i.len() >= UT_RECORDSIZE {
         Vec::from(&i[..UT_RECORDSIZE])
    } else {Vec::from(&i[..]) };
    // let data: Vec<u8> = Vec::from([0u8;384]);
    let (i,ut_type) = take(4u32)(i)?;
    let (i,ut_pid) = take(4u32)(i)?;
    // let (i,ut_line) = map(take(32u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_line) = take(32u32)(i)?;
    // let (i,ut_id) = map(take(4u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_id) = take(4u32)(i)?;
    // let (i,ut_user) = map(take(32u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_user) = take(32u32)(i)?;
    // let (i,ut_host) = map(take(256u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_host) = take(256u32)(i)?;
    let (i,ut_termination) = map(take(2u32), |s: &[u8]| i16::from_ne_bytes(s.try_into().unwrap_or([0u8;2])))(i)?;
    let (i,ut_exit) = map(take(2u32), |s: &[u8]| i16::from_ne_bytes(s.try_into().unwrap_or([0u8;2])))(i)?;
    // let (i,ut_session) = map(take(4u32), |s: &[u8]| i32::from_ne_bytes(s.try_into().unwrap_or([0u8;4])))(i)?;
    let (i,ut_session) = i32(nom::number::Endianness::Native)(i)?;
    let (i,ut_time_sec) = map(take(4u32), |s: &[u8]| u32::from_ne_bytes(s.try_into().unwrap_or([0u8;4])))(i)?;
    let (i,ut_time_usec) = map(take(4u32), |s: &[u8]| u32::from_ne_bytes(s.try_into().unwrap_or([0u8;4])))(i)?;
    // let (i,ut_addr_v6) = count(take(4u32),4)(i)?;
    let (i,ut_addr_v6) = count(map(take(4u32), |s: &[u8]| u32::from_ne_bytes(s.try_into().unwrap_or([0u8;4]))),4)(i)?;
    // let (i,__unused) = map(take(20u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,__unused) = take(20u32)(i)?;

    Ok((i, (data,Utmp{
        ut_type: i32::from_ne_bytes(ut_type.try_into().unwrap_or([0u8;4])),
        // ut_type: i32::from_ne_bytes([2, 0, 0, 0]),
        ut_pid: i32::from_ne_bytes(ut_pid.try_into().unwrap_or([0u8;4])),
        // ut_line: [0; 32],
        ut_line: ut_line.try_into().unwrap(),
        ut_id: ut_id.try_into().unwrap(),
        ut_user: ut_user.try_into().unwrap(),
        ut_host: ut_host.try_into().unwrap(),
        ut_termination: ut_termination,
        ut_exit: ut_exit,
        ut_session: ut_session,
        ut_time_sec: ut_time_sec,
        ut_time_usec: ut_time_usec,
        // ut_addr_v4: ut_addr_v6[0].clone(),
        ut_addr_v6: ut_addr_v6.try_into().unwrap(),
        __unused: __unused.try_into().unwrap(),
    })))
}
pub fn take_one_record(i: &[u8]) -> IResult<&[u8], Utmp> {
    let (i,ut_type) = take(4u32)(i)?;
    let (i,ut_pid) = take(4u32)(i)?;
    // let (i,ut_line) = map(take(32u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_line) = take(32u32)(i)?;
    // let (i,ut_id) = map(take(4u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_id) = take(4u32)(i)?;
    // let (i,ut_user) = map(take(32u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_user) = take(32u32)(i)?;
    // let (i,ut_host) = map(take(256u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,ut_host) = take(256u32)(i)?;
    let (i,ut_termination) = map(take(2u32), |s: &[u8]| i16::from_ne_bytes(s.try_into().unwrap_or([0u8;2])))(i)?;
    let (i,ut_exit) = map(take(2u32), |s: &[u8]| i16::from_ne_bytes(s.try_into().unwrap_or([0u8;2])))(i)?;
    // let (i,ut_session) = map(take(4u32), |s: &[u8]| i32::from_ne_bytes(s.try_into().unwrap_or([0u8;4])))(i)?;
    let (i,ut_session) = i32(nom::number::Endianness::Native)(i)?;
    let (i,ut_time_sec) = map(take(4u32), |s: &[u8]| u32::from_ne_bytes(s.try_into().unwrap_or([0u8;4])))(i)?;
    let (i,ut_time_usec) = map(take(4u32), |s: &[u8]| u32::from_ne_bytes(s.try_into().unwrap_or([0u8;4])))(i)?;
    // let (i,ut_addr_v6) = count(take(4u32),4)(i)?;
    let (i,ut_addr_v6) = count(map(take(4u32), |s: &[u8]| u32::from_ne_bytes(s.try_into().unwrap_or([0u8;4]))),4)(i)?;
    // let (i,__unused) = map(take(20u32),|s: &[u8]| ulity::extract_string(s))(i)?;
    let (i,__unused) = take(20u32)(i)?;

    Ok((i, Utmp{
        ut_type: i32::from_ne_bytes(ut_type.try_into().unwrap_or([0u8;4])),
        // ut_type: i32::from_ne_bytes([2, 0, 0, 0]),
        ut_pid: i32::from_ne_bytes(ut_pid.try_into().unwrap_or([0u8;4])),
        // ut_line: [0; 32],
        ut_line: ut_line.try_into().unwrap(),
        ut_id: ut_id.try_into().unwrap(),
        ut_user: ut_user.try_into().unwrap(),
        ut_host: ut_host.try_into().unwrap(),
        ut_termination: ut_termination,
        ut_exit: ut_exit,
        ut_session: ut_session,
        ut_time_sec: ut_time_sec,
        ut_time_usec: ut_time_usec,
        // ut_addr_v4: ut_addr_v6[0].clone(),
        ut_addr_v6: ut_addr_v6.try_into().unwrap(),
        __unused: __unused.try_into().unwrap(),
    }))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    const UTMPDATA: &[u8] = include_bytes!("../../files4test/wtmp");

    #[test]
    /// test the utmp file: SHA256(../files4test/utmp)= 763b68385d4255c862e9024d021545dec07135b7170f1160f7e00fe3cd5cee2f
    fn test_utmp_parser() {
        // let res = take::<_, _, Error<_>>(384u32)(UTMPDATA);
        let (_,res) = take_one_record(UTMPDATA).ok().unwrap();
        // println!("{:#?}",res);
        assert_eq!(ulity::extract_string(&res.ut_host),"5.4.17-2102.203.6.el8uek.x86_64");

        let (_, res2) = take_all_records(UTMPDATA).ok().unwrap();

        std::io::stdout().flush().unwrap();  // it sames not work fine.
        for (_index, utmp_item) in res2.iter().enumerate() {
            println!("ut_type: {:}\tut_host: {:40}\tut_time_sec: {}",
                     utmp_item.ut_type,
                     ulity::extract_string(&utmp_item.ut_host),
                     utmp_item.ut_time_sec);
        }
        std::io::stdout().flush().unwrap();

        assert_eq!(res2.len(), UTMPDATA.len() / 384);
        // assert_eq!(res2[5].ut_addr_v6, [3649615982,0,0,0]);
    }

    #[test]
    fn test_utmp_asbytes() {
        let utmp0 = ulity::hex_to_bytes("02000000000000007e000000000000000000000000000000000000000000000000000000000000007e7e00007265626f6f740000000000000000000000000000000000000000000000000000352e342e31372d323130322e3230332e362e656c3875656b2e7838365f36340000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e8b42561d7f50e00000000000000000000000000000000000000000000000000000000000000000000000000");
        // let utmp1 = ulity::hex_to_bytes("060000003e0800007474795330000000000000000000000000000000000000000000000000000000747953304c4f47494e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003e08000006b52561217e0900000000000000000000000000000000000000000000000000000000000000000000000000");
        // let utmp2 = ulity::hex_to_bytes("060000001c0800007474793100000000000000000000000000000000000000000000000000000000747479314c4f47494e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001c08000006b52561ecc10a00000000000000000000000000000000000000000000000000000000000000000000000000");
        // let utmp3 = ulity::hex_to_bytes("01000000330000007e000000000000000000000000000000000000000000000000000000000000007e7e000072756e6c6576656c000000000000000000000000000000000000000000000000352e342e31372d323130322e3230332e362e656c3875656b2e7838365f3634000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000021b525614a780c00000000000000000000000000000000000000000000000000000000000000000000000000");
        // let utmp4 = ulity::hex_to_bytes("070000008f811c007074732f3000000000000000000000000000000000000000000000000000000074732f306f706300000000000000000000000000000000000000000000000000000000003131302e3138342e3133362e32313700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000095d1f562ce6704006eb888d90000000000000000000000000000000000000000000000000000000000000000");
        // let utmp5 = ulity::hex_to_bytes("07000000ba131d007074732f3100000000000000000000000000000000000000000000000000000074732f316f706300000000000000000000000000000000000000000000000000000000003131302e3138342e3133362e323137000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e4faf562c6f405006eb888d90000000000000000000000000000000000000000000000000000000000000000");
        let (_, res2) = take_all_records(UTMPDATA).ok().unwrap();
        for (index, utmp_item) in res2.iter().enumerate() {
            println!("{:03}: {:?}",index, utmp_item);
            match index {
                0 => assert_eq!(&utmp_item.as_bytes_vec(),utmp0.as_ref().unwrap()),
                // 1 => assert_eq!(utmp_item.as_bytes(),utmp1.as_ref().unwrap().as_slice()),
                // 2 => assert_eq!(utmp_item.as_bytes(),utmp2.as_ref().unwrap().as_slice()),
                // 3 => assert_eq!(utmp_item.as_bytes(),utmp3.as_ref().unwrap().as_slice()),
                // 4 => assert_eq!(utmp_item.as_bytes(),utmp4.as_ref().unwrap().as_slice()),
                5 => assert_eq!(utmp_item.as_bytes(),&UTMPDATA[384*5..384*6]),
                i => assert_eq!(utmp_item.as_bytes(),&UTMPDATA[384*i..384*(i+1)]),
                // _ => (),
            }
        }
    }

    #[test]
    fn test_utmp_record_debug_output() {
        let (_,res) = take_one_record(UTMPDATA).ok().unwrap();
        println!("{:#?}",res);
    }

    #[test]
    fn test_ulity_hex2bytes() {
        let res = ulity::hex_to_bytes("0x3230332e362e656c3875656b2e783836");
        println!("{:?}", res);
        assert_eq!(res, Some(vec![50, 48, 51, 46, 54, 46, 101, 108, 56, 117, 101, 107, 46, 120, 56, 54]));
    }
    //
    // #[test]
    // fn test_ulity_hex2bytes() {
    //     let res = ulity::hex_to_bytes("0x3230332e362e656c3875656b2e783836");
    //     println!("{:?}", res);
    //     assert_eq!(res, Some(vec![50, 48, 51, 46, 54, 46, 101, 108, 56, 117, 101, 107, 46, 120, 56, 54]));
    // }

}