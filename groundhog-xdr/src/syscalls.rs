//! # Syscalls Module
//!
//! This contains the generated system call bindings for all syscalls in groundhog.
//! In most cases, you should not need to call these functions directly.  Instead,
//! the SDK provides a higher-level interface for using these functions.
//!
//! Internal notes, for those who are curious:
//!
//! There are three variants generated for each syscall:
//!
//! 1. **{syscall_name}**: Convenience wrapper that returns `SyscallResult<T>`.
//!    Takes only input parameters and returns outputs in the Result. Calls `uninit_{syscall_name}`
//!    internally with MaybeUninit outputs. For syscalls that don't return (like Exit, Return),
//!    this variant returns `!` and calls the uninit variant directly.
//!    We do not generate this method if it returns a span argument,
//!    as in that case the caller is responsible for providing a buffer into which the
//!    syscall can write the output.
//!
//! 2. **raw_{syscall_name}**: Takes input arguments as immutable references (&) or,
//!    for trivially copyable types that fit within a u64, as the value itself.
//!    Outputs are taken as mutable references (&mut). Returns a `SysErr` error code.
//!
//! 3. **uninit_{syscall_name}**: Low-level variant that takes outputs as
//!    `&mut MaybeUninit<T>` arguments (instead of `&mut T`).
//!    The output variables are initialized only if the error is `kSyscallErrorCodeNone` (0).
//!
//! The raw_{syscall_name} variant is a thin wrapper around the uninit_{syscall_name} variant,
//! with appropriate transmute calls to convert from `&mut T` to `&mut MaybeUninit<T>`.
//! The {syscall_name} variant also wraps uninit_{syscall_name}, handling MaybeUninit initialization
//! and returning results in a more idiomatic Rust style.
//!
//! These bindings are a thin wrapper around the webassembly imported function
//! that is invoked to leave a webassembly module (and execute a system call).
//! That function (here, `syscall_wrapper`) is imported as the webassembly module `marmot` function name `syscall`.
//! You should never need to invoke `syscall_wrapper` directly (which is why that function is not a public member of this module).
//! Instead, use the typed bindings as generated in this module.  
//! Under the hood, these bindings encode their arguments as either integers or pointers into WebAssembly memory.  

use crate::*;
use core::fmt;
use thiserror::Error;

#[cfg_attr(feature = "xdrs", derive(xdrs::ReadXDR, xdrs::WriteXDR))]
#[cfg_attr(feature = "spec", derive(xdrs::XDR))]
#[derive(Error, Copy, Clone, PartialEq, Eq)]
#[error("{:?}", self)]
#[repr(transparent)]
pub struct SysErr(pub i32);

pub type SyscallResult<T> = core::result::Result<T, SysErr>;

impl SysErr {
    pub fn is_code(&self, code: SyscallErrorCode) -> bool {
        self.0 == code.0
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

    pub fn is_invalid_input(self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput)
    }

    pub fn is_aborted(self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeFrameReverted)
    }
}

impl fmt::Debug for SysErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = SyscallErrorCode(self.0);
        match code {
            SyscallErrorCode::kSyscallErrorCodeNone => {
                write!(f, "SysErr<None>")
            }
            SyscallErrorCode::kSyscallErrorCodeFault => {
                write!(f, "SysErr<Fault>")
            }
            SyscallErrorCode::kSyscallErrorCodeNoSyscall => {
                write!(f, "SysErr<NoSyscall>")
            }
            SyscallErrorCode::kSyscallErrorCodeIsRootTransaction => {
                write!(f, "SysErr<IsRootTransaction>")
            }
            SyscallErrorCode::kSyscallErrorCodeContractAlreadyDeployed => {
                write!(f, "SysErr<ContractAlreadyDeployed>")
            }
            SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable => {
                write!(f, "SysErr<NoReturnBufferAvailable>")
            }
            SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput => {
                write!(f, "SysErr<InvalidSyscallInput>")
            }
            SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist => {
                write!(f, "SysErr<StorageKeyNotExist>")
            }
            SyscallErrorCode::kSyscallErrorCodeNoRetrocausalQueueEntry => {
                write!(f, "SysErr<NoRetrocausalQueueEntry>")
            }
            SyscallErrorCode::kSyscallErrorCodeDeterministicWasm => {
                write!(f, "SysErr<DeterministicWasm>")
            }
            SyscallErrorCode::kSyscallErrorCodeFrameReverted => {
                write!(f, "SysErr<FrameReverted>")
            }
            _ => write!(f, "SysErr({})", self.0),
        }
    }
}

impl From<SyscallErrorCode> for SysErr {
    fn from(code: SyscallErrorCode) -> Self {
        SysErr(code.0)
    }
}

impl From<SysErr> for i32 {
    fn from(err: SysErr) -> Self {
        err.0
    }
}

// Note: On MacOS accidentally building with this link_name silently links against
// some system "syscall" symbol and gives garbage results.
#[cfg(all(
    not(feature = "syscalls_mock"),
    not(target_family = "wasm"),
    not(docsrs)
))]
compile_error!("do not link wasm syscall binding on non-wasm targets!");

#[cfg(not(feature = "syscalls_mock"))]
#[link(wasm_import_module = "marmot")]
unsafe extern "C" {
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
    ) -> i32;
}

#[cfg(feature = "syscalls_mock")]
unsafe extern "C-unwind" {
    fn syscall_wrapper_impl(
        syscall_no: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
        arg7: u64,
    ) -> i32;
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
        SysErr(syscall_wrapper_impl(
            syscall_no.0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
            arg7,
        ))
    }
}

groundhog_xdr_macros::generate_syscall_decls!("src/syscalls.yaml");
