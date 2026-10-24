//! # Syscalls Module
//!
//! This contains the generated system call bindings for all syscalls in Thruput.
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
//!    for non-optional arguments marked `primitive_type` in syscalls.yaml, as the
//!    value itself; such a type must fit in one `SyscallWord` (32 bits on riscv32).
//!    Outputs are taken as mutable references (&mut). Returns a `SysErr` error code.
//!
//! 3. **uninit_{syscall_name}**: Low-level variant that takes outputs as
//!    `&mut MaybeUninit<T>` arguments (instead of `&mut T`), and inputs the same way
//!    as the raw variant. Fixed-size outputs are initialized only if the error is
//!    `kSyscallErrorCodeNone` (0); span outputs only in their first `min(len, written_len)` bytes.
//!
//! The raw_{syscall_name} variant is a thin wrapper around the uninit_{syscall_name} variant,
//! with appropriate transmute calls to convert from `&mut T` to `&mut MaybeUninit<T>`.
//! The {syscall_name} variant also wraps uninit_{syscall_name}, handling MaybeUninit initialization
//! and returning results in a more idiomatic Rust style.
//!
//! These bindings are a thin wrapper around the dispatch chokepoint
//! `thruput_syscall::syscall_wrapper`, which leaves the guest and executes a
//! system call. See the `thruput-syscall` crate for the per-target dispatch
//! paths (riscv32 `ecall`, native mock).

use crate::*;

// The `syscalls` feature pulls in `thruput-syscall`, which only provides a
// dispatch path on riscv32 or under its `syscalls_mock` feature. On any other
// target it is empty, so guard with a clear message rather than letting the
// `SyscallWord` / `syscall_wrapper` imports below fail as unresolved.
#[cfg(not(any(target_arch = "riscv32", feature = "syscalls_mock")))]
compile_error!(
    "the `syscalls` feature has no dispatch path on this target; build for riscv32 or enable `syscalls_mock`"
);

pub use thruput_syscall::SyscallWord;
use thruput_syscall::syscall_wrapper;

pub type SyscallResult<T> = core::result::Result<T, SysErr>;

thruput_xdr_macros::generate_syscall_decls!("src/syscalls.yaml");
