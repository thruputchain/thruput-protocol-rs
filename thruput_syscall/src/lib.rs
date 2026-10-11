//! # Thruput syscall dispatch
//!
//! The raw guest-exit boundary for Thruput syscalls. This crate contains the
//! single dispatch chokepoint, `syscall_wrapper`, which leaves the guest and
//! executes a system call. It speaks only in integer words (`SyscallWord`) and
//! raw `i32` error codes — the typed `SyscallNo` / `SysErr` layer lives in
//! `thruput_xdr`.
//!
//! A dispatch path exists only when the target is `riscv32`, or when the
//! `syscalls_mock` feature is enabled. On any other target (a plain host
//! without the mock) there is no way to leave the guest, so the dispatch
//! chokepoint `syscall_wrapper` is not exported. This keeps a feature-less host
//! build (and `cargo build -p`, `cargo hack`, rust-analyzer) clean without a
//! `compile_error!` guard. The `SyscallWord` type alias, by contrast, is a pure
//! pointer-width type with no dispatch dependency, so it is always defined.
//!
//! There are two guest dispatch paths, selected by target:
//!
//! - **riscv32**: `syscall_wrapper` issues the RISC-V `ecall` trap instruction
//!   directly, passing the syscall number in `a0` and arguments in `a1..a7`
//!   (the host returns the error code in `a0`).
//! - **native mock** (`syscalls_mock`): `syscall_wrapper` calls a native
//!   `syscall_wrapper_impl` symbol, used by `native_exec` in the SDK and by the
//!   syscall tests.
//!
//! You should never need to invoke `syscall_wrapper` directly; instead use the
//! typed bindings generated in `thruput_xdr`.
#![no_std]

/// The integer width of a single syscall argument slot.
///
/// Syscall arguments are passed as pointer-width words. On `riscv32` each slot
/// is a 32-bit value, carried in a register (`a1..a7`) across the `ecall` trap.
/// On native targets each slot is a 64-bit value, wide enough to carry a host
/// pointer (this is the path used by `native_exec` in the SDK). This happens to
/// equal the platform pointer width, but is spelled out explicitly so the
/// syscall ABI is decoupled from any incidental use of `usize` elsewhere.
///
/// This is a pure type alias with no dispatch dependency, so it is always
/// defined — even on targets where `syscall_wrapper` is unavailable.
#[cfg(target_arch = "riscv32")]
pub type SyscallWord = u32;
#[cfg(not(target_arch = "riscv32"))]
pub type SyscallWord = u64;

// On riscv32 the slot must be exactly pointer/register width (32 bits); a 64-bit
// slot would be split across a register pair and break the `ecall` ABI.
#[cfg(target_arch = "riscv32")]
const _: () = assert!(core::mem::size_of::<SyscallWord>() == 4);

#[cfg(any(target_arch = "riscv32", feature = "syscalls_mock"))]
pub use dispatch::syscall_wrapper;

#[cfg(any(target_arch = "riscv32", feature = "syscalls_mock"))]
mod dispatch {
    use super::SyscallWord;

    // The syscall entrypoint is declared with the *concrete* slot type for each
    // target, written out explicitly rather than via the `SyscallWord` alias. Since
    // `syscall_wrapper` (below) passes `SyscallWord`-typed values into it, any
    // disagreement between these declarations and `SyscallWord` is a compile error
    // at the call site — a tripwire if `SyscallWord` is ever defined incorrectly.

    // Native mock entrypoint (used by `native_exec` and the syscall tests): always
    // native, so the slots are concretely 64-bit.
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

    // riscv32 (bare-metal): no host import — issue the `ecall` trap directly. The
    // syscall number is passed in `a0` and arguments in `a1..a7`; the host leaves
    // the error code in `a0`. Per the RISC-V calling convention `a0..a7` are
    // caller-saved argument registers, so each is declared as clobbered
    // (`inlateout(reg) val => _`), except `a0`, whose post-state is the result.
    // The `u32` parameters match `SyscallWord` on riscv32; if `SyscallWord` were
    // ever mis-defined wider, the `syscall_wrapper` call site would fail to compile.
    #[cfg(all(not(feature = "syscalls_mock"), target_arch = "riscv32"))]
    #[inline]
    unsafe fn syscall_wrapper_impl(
        syscall_no: u32,
        arg1: u32,
        arg2: u32,
        arg3: u32,
        arg4: u32,
        arg5: u32,
        arg6: u32,
        arg7: u32,
    ) -> i32 {
        let result: i32;
        // SAFETY: `ecall` traps to the host with the documented register ABI and
        // touches neither the stack nor memory through the stack pointer.
        unsafe {
            core::arch::asm!(
                "ecall",
                inlateout("a0") syscall_no => result,
                inlateout("a1") arg1 => _,
                inlateout("a2") arg2 => _,
                inlateout("a3") arg3 => _,
                inlateout("a4") arg4 => _,
                inlateout("a5") arg5 => _,
                inlateout("a6") arg6 => _,
                inlateout("a7") arg7 => _,
                options(nostack),
            );
        }
        result
    }

    /// The single dispatch chokepoint: leaves the guest and executes a syscall.
    ///
    /// Takes the syscall number and arguments as raw pointer-width words and returns
    /// the raw `i32` error code. The typed `SyscallNo` / `SysErr` wrapping is applied
    /// by the generated bindings in `thruput_xdr`.
    #[inline(always)]
    pub fn syscall_wrapper(
        syscall_no: SyscallWord,
        arg1: SyscallWord,
        arg2: SyscallWord,
        arg3: SyscallWord,
        arg4: SyscallWord,
        arg5: SyscallWord,
        arg6: SyscallWord,
        arg7: SyscallWord,
    ) -> i32 {
        // SAFETY: `syscall_wrapper_impl` requires only correctly-typed integer
        // arguments (it has no further preconditions); the typed `SyscallWord`
        // parameters guarantee that.
        unsafe {
            syscall_wrapper_impl(
                syscall_no, arg1, arg2, arg3, arg4, arg5, arg6, arg7,
            )
        }
    }
}
