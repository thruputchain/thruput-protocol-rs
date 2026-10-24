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

/// Mock syscall handler that captures invocations
pub struct MockSyscallHandler {
    invocations: Arc<Mutex<Vec<SyscallInvocation>>>,
    return_value: Arc<Mutex<SysErr>>,
}

impl MockSyscallHandler {
    pub fn new() -> Self {
        Self {
            invocations: Arc::new(Mutex::new(Vec::new())),
            return_value: Arc::new(Mutex::new(SysErr(0))), // Default to success
        }
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
        *self.return_value.lock().unwrap()
    }
}

// Thread-local mock handler - each test thread gets its own handler
thread_local! {
    static MOCK_HANDLER: RefCell<Option<Arc<MockSyscallHandler>>> = RefCell::new(None);
}

/// Installs `handler` for syscalls made on the current thread. Syscalls on a
/// thread with no handler succeed without being recorded.
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
        // Default: return success if no handler is set
        SysErr(0)
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
