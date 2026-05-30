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

pub type SyscallResult<T> = core::result::Result<T, SysErr>;

/// The integer width of a single syscall argument slot.
///
/// Syscall arguments are passed as pointer-width words. On `wasm32` each slot is
/// a 32-bit value, lowered to a wasm `i32` to match the host (`marmot`) ABI. On
/// native targets each slot is a 64-bit value, wide enough to carry a host
/// pointer (this is the path used by `native_exec` in the SDK). This happens to
/// equal the platform pointer width, but is spelled out explicitly so the
/// syscall ABI is decoupled from any incidental use of `usize` elsewhere.
#[cfg(target_arch = "wasm32")]
pub type SyscallWord = u32;
#[cfg(not(target_arch = "wasm32"))]
pub type SyscallWord = u64;

// Note: On MacOS accidentally building with this link_name silently links against
// some system "syscall" symbol and gives garbage results.
#[cfg(all(
    not(feature = "syscalls_mock"),
    not(target_family = "wasm"),
    not(docsrs)
))]
compile_error!("do not link wasm syscall binding on non-wasm targets!");

// The syscall entrypoint is declared with the *concrete* slot type for each
// target, written out explicitly rather than via the `SyscallWord` alias. Since
// `syscall_wrapper` (below) passes `SyscallWord`-typed values into it, any
// disagreement between these declarations and `SyscallWord` is a compile error
// at the call site — a tripwire if `SyscallWord` is ever defined incorrectly.

// wasm32 real import, provided by the host (`marmot`): 32-bit slots, lowered to
// wasm `i32`.
#[cfg(all(not(feature = "syscalls_mock"), target_arch = "wasm32"))]
#[link(wasm_import_module = "marmot")]
unsafe extern "C" {
    #[link_name = "syscall"]
    fn syscall_wrapper_impl(
        syscall_no: u32,
        arg1: u32,
        arg2: u32,
        arg3: u32,
        arg4: u32,
        arg5: u32,
        arg6: u32,
        arg7: u32,
    ) -> i32;
}

// Non-wasm32 without the mock: only reachable under `--cfg docsrs` (otherwise
// the compile_error above fires). Type-checked for the docs build but never
// linked. 64-bit slots, matching the native `SyscallWord`.
#[cfg(all(not(feature = "syscalls_mock"), not(target_arch = "wasm32")))]
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

#[inline(always)]
fn syscall_wrapper(
    syscall_no: SyscallNo,
    arg1: SyscallWord,
    arg2: SyscallWord,
    arg3: SyscallWord,
    arg4: SyscallWord,
    arg5: SyscallWord,
    arg6: SyscallWord,
    arg7: SyscallWord,
) -> SysErr {
    unsafe {
        SysErr(syscall_wrapper_impl(
            syscall_no.0 as SyscallWord,
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
