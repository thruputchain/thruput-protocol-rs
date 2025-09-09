use crate::*;
use thiserror::Error;

macros::generate_syscall_numbers!("../protocol/syscalls.yaml");

#[derive(xdrs::ReadXDR, xdrs::WriteXDR)]
#[cfg_attr(feature = "spec", derive(xdrs::XDR))]
#[derive(Error, Copy, Clone, Debug, PartialEq, Eq)]
#[error("SysErr({0})")]
#[repr(transparent)]
pub struct SysErr(pub i32);
pub type SyscallResult<T> = core::result::Result<T, SysErr>;

impl SysErr {
    pub fn is_code(&self, code: SyscallErrorCode) -> bool {
        self.0 == code as i32
    }

    pub fn is_none(&self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeNone)
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_storage_key_not_exist(self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist)
    }

    pub fn is_aborted(self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeFrameReverted)
    }
}

impl From<SyscallErrorCode> for SysErr {
    fn from(code: SyscallErrorCode) -> Self {
        SysErr(code as i32)
    }
}

impl From<SysErr> for i32 {
    fn from(err: SysErr) -> Self {
        err.0
    }
}

#[cfg(not(feature = "syscalls_mock"))]
#[link(wasm_import_module = "marmot")]
extern "C" {
    #[link_name = "syscall"]
    fn syscall_wrapper_impl(
        syscall_no: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
        arg7: u64,
    ) -> SysErr;
}

#[cfg(feature = "syscalls_mock")]
extern "C-unwind" {
    fn syscall_wrapper_impl(
        syscall_no: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
        arg7: u64,
    ) -> SysErr;
}

fn syscall_wrapper(
    syscall_no: SyscallNo,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
) -> SysErr {
    unsafe {
        syscall_wrapper_impl(
            syscall_no as u64, arg1, arg2, arg3, arg4, arg5, arg6, arg7,
        )
    }
}

macros::generate_syscall_decls!("../protocol/syscalls.yaml");
