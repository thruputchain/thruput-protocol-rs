use crate::ffi_enums::{HostFnErrorCode, InvokeErrorCode, SystemErrorCode};

impl From<SystemErrorCode> for HostFnErrorCode {
    fn from(error_code: SystemErrorCode) -> Self {
        match error_code {
            SystemErrorCode::Unrecoverable => HostFnErrorCode::Unrecoverable,
            SystemErrorCode::Abandon => HostFnErrorCode::Abandon,
            SystemErrorCode::ValidationFailure => {
                HostFnErrorCode::ValidationFailure
            }
            _ => HostFnErrorCode::Unrecoverable,
        }
    }
}

impl From<SystemErrorCode> for InvokeErrorCode {
    fn from(error_code: SystemErrorCode) -> Self {
        match error_code {
            SystemErrorCode::Unrecoverable => InvokeErrorCode::Unrecoverable,
            SystemErrorCode::Abandon => InvokeErrorCode::Abandon,
            SystemErrorCode::ValidationFailure => {
                InvokeErrorCode::ValidationFailure
            }
            _ => InvokeErrorCode::Unrecoverable,
        }
    }
}
