//! Mock implementation of syscall_wrapper_impl for testing
//! This provides a way to capture and verify syscall arguments
//! Uses thread-local storage so tests can run in parallel without interference

use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use thruput_xdr::syserr::SysErr;

/// Captured syscall invocation
#[derive(Debug, Clone)]
pub struct SyscallInvocation {
    pub syscall_no: u32,
    pub args: [u64; 7],
}

/// Answers a syscall in place of the runtime: given the syscall number and
/// its arguments, it writes any outputs (see [`write_output`]) and returns
/// the syscall's error code.
pub type Responder = Box<dyn Fn(u32, &[u64; 7]) -> SysErr + Send + Sync>;

/// Mock syscall handler that captures invocations.
///
/// Without a responder, a syscall returns the configured return value and
/// leaves its outputs untouched, as the `raw_*` tests expect: they pass
/// initialized output buffers. The convenience wrappers read their outputs
/// after a successful syscall, so testing one needs a responder that writes
/// them, as the runtime would.
pub struct MockSyscallHandler {
    invocations: Arc<Mutex<Vec<SyscallInvocation>>>,
    return_value: Arc<Mutex<SysErr>>,
    responder: Mutex<Option<Responder>>,
}

impl MockSyscallHandler {
    pub fn new() -> Self {
        Self {
            invocations: Arc::new(Mutex::new(Vec::new())),
            return_value: Arc::new(Mutex::new(SysErr(0))), // Default to success
            responder: Mutex::new(None),
        }
    }

    /// Answers every later syscall with `responder` instead of the configured
    /// return value.
    pub fn set_responder(
        &self,
        responder: impl Fn(u32, &[u64; 7]) -> SysErr + Send + Sync + 'static,
    ) {
        *self.responder.lock().unwrap() = Some(Box::new(responder));
    }

    pub fn set_return_value(&self, err: SysErr) {
        *self.return_value.lock().unwrap() = err;
    }

    pub fn get_invocations(&self) -> Vec<SyscallInvocation> {
        self.invocations.lock().unwrap().clone()
    }

    pub fn clear_invocations(&self) {
        self.invocations.lock().unwrap().clear();
    }

    pub fn get_last_invocation(&self) -> Option<SyscallInvocation> {
        self.invocations.lock().unwrap().last().cloned()
    }

    fn record_invocation(&self, syscall_no: u32, args: [u64; 7]) -> SysErr {
        self.invocations
            .lock()
            .unwrap()
            .push(SyscallInvocation { syscall_no, args });
        match self.responder.lock().unwrap().as_ref() {
            Some(responder) => responder(syscall_no, &args),
            None => *self.return_value.lock().unwrap(),
        }
    }
}

/// Writes `value` through `arg`, an output argument of a syscall, as the
/// runtime does when the syscall succeeds.
///
/// # Safety
///
/// `arg` must be the pointer the syscall received for an output of type `T`.
pub unsafe fn write_output<T>(arg: u64, value: T) {
    // SAFETY: the caller guarantees `arg` points to a writable `T`.
    unsafe { (arg as *mut T).write(value) }
}

// Thread-local mock handler - each test thread gets its own handler
thread_local! {
    static MOCK_HANDLER: RefCell<Option<Arc<MockSyscallHandler>>> = RefCell::new(None);
}

/// Installs `handler` for syscalls made on the current thread. A syscall on a
/// thread with no handler panics: succeeding without writing its outputs
/// would let a convenience wrapper read uninitialized memory.
pub fn set_mock_handler(handler: Arc<MockSyscallHandler>) {
    MOCK_HANDLER.with(|h| {
        *h.borrow_mut() = Some(handler);
    });
}

fn get_mock_handler() -> Option<Arc<MockSyscallHandler>> {
    MOCK_HANDLER.with(|h| h.borrow().clone())
}

/// Mock implementation of syscall_wrapper_impl
/// This is called by `thruput_syscall::syscall_wrapper` under the
/// `syscalls_mock` feature, which the generated syscall code calls
#[no_mangle]
pub extern "C-unwind" fn syscall_wrapper_impl(
    syscall_no: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
) -> SysErr {
    if let Some(handler) = get_mock_handler() {
        handler.record_invocation(
            syscall_no as u32,
            [arg1, arg2, arg3, arg4, arg5, arg6, arg7],
        )
    } else {
        panic!(
            "syscall {syscall_no} made on a thread with no mock handler; \
             call set_mock_handler first"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_handler_basic() {
        let handler = Arc::new(MockSyscallHandler::new());
        set_mock_handler(handler.clone());

        let result = syscall_wrapper_impl(1, 2, 3, 4, 5, 6, 7, 8);
        assert!(result.is_none());

        let invocations = handler.get_invocations();
        assert_eq!(invocations.len(), 1);
        assert_eq!(invocations[0].syscall_no, 1);
        assert_eq!(invocations[0].args, [2, 3, 4, 5, 6, 7, 8]);
    }
}
