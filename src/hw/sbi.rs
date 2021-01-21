mod ext {
    pub const SYSTEM_RESET: usize = 0x53525354;
}

pub const RST_SHUTDOWN: usize = 0;
// pub const RST_REBOOT_COLD: usize = 1;
// pub const RST_REBOOT_WARM: usize = 2;
pub const RST_NO_REASON: usize = 0;
pub const RST_SYSTEM_FAILURE: usize = 1;

#[inline]
unsafe fn sbi_call(
    eid: usize,
    fid: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> (isize, isize) {
    let (error, value);
    asm!(
        "ecall",
        in("a7") eid,
        in("a6") fid,
        in("a0") arg0,
        in("a1") arg1,
        in("a2") arg2,
        lateout("a0") error,
        lateout("a1") value,
    );
    (error, value)
}

pub fn system_reset(kind: usize, reason: usize) {
    let (error, _) = unsafe { sbi_call(ext::SYSTEM_RESET, 0, kind, reason, 0) };
    crate::println!(
        "Warning: system reset through SBI failed: error code {}",
        error
    );
}
