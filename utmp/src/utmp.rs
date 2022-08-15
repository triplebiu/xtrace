use crate::{UT_HOSTSIZE, UT_LINESIZE, UT_NAMESIZE};

/// 参考来自：https://github.com/libyal/dtformats/blob/main/documentation/Utmp%20login%20records%20format.asciidoc
#[repr(C)]
#[derive(Clone, Debug)]
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
    pub ut_line: String,
    /// Terminal indentifier, Terminal name suffix, or `inittab(5)` ID
    /// String, 4 bytes.
    pub ut_id: String,
    /// Username
    /// Contains an encoded string, which can be empty (seen in combination with DEAD_PROCESS)
    /// String, 32 bytes.
    pub ut_user: String,
    /// Hostname for remote login, or kernel version for run-level message
    /// Contains an encoded string, which can be empty (seein in combination with LOGIN_PROCESS) or contain other data such as "4.15.3-300.fc27.x86_64" or "/dev/tty2"
    /// String, 256 bytes.
    pub ut_host: String,
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
    pub ut_addr_v4: u32,
    /// Reserved for future use
    /// `[0u8; 20]`, 20 bytes.
    pub __unused: String,
}

#[test]
fn test_utmp_record_block_size(){
    assert_eq!(std::mem::size_of::<Utmp>(), 384)    // todo: utmp struct have problem.
}