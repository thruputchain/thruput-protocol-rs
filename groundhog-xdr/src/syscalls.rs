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
//! These bindings are a thin wrapper around the dispatch chokepoint
//! `groundhog_syscall::syscall_wrapper`, which leaves the guest and executes a
//! system call. See the `groundhog-syscall` crate for the per-target dispatch
//! paths (wasm `marmot` import, riscv32 `ecall`, native mock).

use crate::*;

pub use groundhog_syscall::SyscallWord;
use groundhog_syscall::syscall_wrapper;

pub type SyscallResult<T> = core::result::Result<T, SysErr>;

groundhog_xdr_macros::generate_syscall_decls!("src/syscalls.yaml");
