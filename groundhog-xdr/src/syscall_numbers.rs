groundhog_xdr_macros::generate_syscall_numbers!("src/syscalls.yaml");

impl From<i32> for SyscallNo {
    fn from(value: i32) -> Self {
        Self(value as u32)
    }
}
