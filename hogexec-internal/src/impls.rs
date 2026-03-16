use crate::ffi_enums::{HostFnErrorCode, InvokeErrorCode, SystemErrorCode};

impl From<SystemErrorCode> for HostFnErrorCode {
    fn from(error_code: SystemErrorCode) -> Self {
        match error_code {
            SystemErrorCode::kSystemUnrecoverable => {
                HostFnErrorCode::kHostUnrecoverable
            }
            SystemErrorCode::kSystemAbortTransaction => {
                HostFnErrorCode::kHostAbortTransaction
            }
            SystemErrorCode::kSystemValidationFailure => {
                HostFnErrorCode::kHostValidationFailure
            }
            _ => HostFnErrorCode::kHostUnrecoverable,
        }
    }
}

impl From<SystemErrorCode> for InvokeErrorCode {
    fn from(error_code: SystemErrorCode) -> Self {
        match error_code {
            SystemErrorCode::kSystemUnrecoverable => {
                InvokeErrorCode::kInvokeUnrecoverable
            }
            SystemErrorCode::kSystemAbortTransaction => {
                InvokeErrorCode::kInvokeAbortTransaction
            }
            SystemErrorCode::kSystemValidationFailure => {
                InvokeErrorCode::kInvokeValidationFailure
            }
            _ => InvokeErrorCode::kInvokeUnrecoverable,
        }
    }
}
