use crate::{UT_HOSTSIZE, UT_LINESIZE, UT_NAMESIZE};
use std::{fmt};
use nom::AsBytes;
use std::convert::TryInto;
use super::ulity;



// 参考来自：https://github.com/libyal/dtformats/blob/main/documentation/Utmp%20login%20records%20format.asciidoc
/// Linux utmp login record format.
#[repr(C)]
#[derive(Clone)]
pub struct Utmp {
    /// Type of record
    /// i32, 4 bytes.
    pub ut_type: i32,
    /// Process identifier (PID)
    /// i32, 4 bytes.
    pub ut_pid: i32,
    /// Terminal
    /// Contains an encoded string, which can be "\~" in combination with an username of "shutdown", "reboot" or "runlevel"
    /// String, 32 bytes.
    // pub ut_line: [u8; UT_LINESIZE],
    pub ut_line: [u8; UT_LINESIZE],
    /// Terminal indentifier, Terminal name suffix, or `inittab(5)` ID
    /// String, 4 bytes.
    pub ut_id: [u8; 4],
    /// Username
    /// Contains an encoded string, which can be empty (seen in combination with DEAD_PROCESS)
    /// String, 32 bytes.
    pub ut_user: [u8; UT_NAMESIZE],
    /// Hostname for remote login, or kernel version for run-level message
    /// Contains an encoded string, which can be empty (seein in combination with LOGIN_PROCESS) or contain other data such as "4.15.3-300.fc27.x86_64" or "/dev/tty2"
    /// String, 256 bytes.
    pub ut_host: [u8; UT_HOSTSIZE],
    /// Termination status
    /// i16, 2 bytes.
    pub ut_termination: i16,
    /// Exit status
    /// i16, 2 bytes.
    pub ut_exit: i16,
    /// Session ID (`getsid(2)`) used for windowing
    /// i32, 4 bytes.
    pub ut_session: i32,
    /// Timestamp
    /// u32, 4 bytes.
    pub ut_time_sec: u32,
    /// Microseconds
    /// u32, 4 bytes.
    pub ut_time_usec: u32,
    /// Internet address of remote host; IPv4 address uses just `ut_addr_v6[0]`
    /// `[u32; 4]`, 16 bytes.
    pub ut_addr_v6: [u32; 4],
    // pub ut_addr_v4: u32,
    /// Reserved for future use
    /// `[u8; 20]`, 20 bytes.
    pub __unused: [u8; 20],
}


impl fmt::Debug for Utmp {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Utmp")
            .field("ut_type", &self.ut_type)
            .field("ut_pid", &self.ut_pid)
            .field("ut_line", &ulity::extract_string(&self.ut_line))
            .field("ut_id", &ulity::extract_string(&self.ut_id))
            .field("ut_user", &ulity::extract_string(&self.ut_user))
            .field("ut_host", &ulity::extract_string(&self.ut_host))
            .field("ut_termination", &self.ut_termination)
            .field("ut_exit", &self.ut_exit)
            .field("ut_session", &self.ut_session)
            .field("ut_time_sec", &self.ut_time_sec)
            .field("ut_time_usec", &self.ut_time_usec)
            .field("ut_addr_v6", &self.ut_addr_v6)
            .field("__unused", &ulity::extract_string(&self.__unused))
            .finish()
    }
}

impl Utmp {
    pub fn as_bytes(&self) -> [u8;384] {
        // (*self).as_bytes()
        let mut outvec = Vec::new();
        outvec.extend_from_slice(&self.ut_type.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_pid.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_line.as_bytes());
        outvec.extend_from_slice(&self.ut_id.as_bytes());
        outvec.extend_from_slice(&self.ut_user.as_bytes());
        outvec.extend_from_slice(&self.ut_host.as_bytes());
        outvec.extend_from_slice(&self.ut_termination.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_exit.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_session.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_time_sec.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_time_usec.to_ne_bytes());
        // outvec.extend_from_slice(&self.ut_addr_v6.iter()
        //     .map(|a|a.to_ne_bytes())
        //     .flatten()
        //     .collect::<Vec<u8>>()
        //     .try_into()    // 此处不好进行类型声明。
        //     .unwrap()
        // );
        for i in 0..self.ut_addr_v6.len() {
            outvec.extend_from_slice(&self.ut_addr_v6[i].to_ne_bytes());
        }
        // outvec.extend_from_slice(&self.ut_addr_v6.into());
        // outvec.extend_from_slice(unsafe{mem::transmute_copy(&self.ut_addr_v6)});
        // outvec.extend_from_slice(&[0u8; 16]);
        // let mut tmpvec:Vec<u8> = Vec::new();
        // outvec.append({
        //     for item in &self.ut_addr_v6 {
        //         let tmp:[u8;4] = item.to_ne_bytes();
        //         println!("{:?}",tmp);
        //         tmpvec.extend_from_slice(&tmp);
        //     }
        //     // tmpvec.into()
        //     &mut tmpvec
        //     // let ipv6bytes: [u8;16] = tmpvec.try_into().unwrap();
        //     // ipv6bytes
        // });

        outvec.extend_from_slice(&[0u8; 20]);
        // println!("outvec lenght: {}",outvec.len());
        outvec.try_into().unwrap()
    }

    pub fn as_bytes_vec(&self) -> Vec<u8> {
        let mut outvec = Vec::new();
        outvec.extend_from_slice(&self.ut_type.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_pid.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_line.as_bytes());
        outvec.extend_from_slice(&self.ut_id.as_bytes());
        outvec.extend_from_slice(&self.ut_user.as_bytes());
        outvec.extend_from_slice(&self.ut_host.as_bytes());
        outvec.extend_from_slice(&self.ut_termination.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_exit.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_session.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_time_sec.to_ne_bytes());
        outvec.extend_from_slice(&self.ut_time_usec.to_ne_bytes());
        for i in 0..self.ut_addr_v6.len() {
            outvec.extend_from_slice(&self.ut_addr_v6[i].to_ne_bytes());
        }
        outvec.extend_from_slice(&[0u8; 20]);
        outvec
    }
}

#[test]
fn test_utmp_record_block_size() {
    assert_eq!(std::mem::size_of::<Utmp>(), 384);
}

