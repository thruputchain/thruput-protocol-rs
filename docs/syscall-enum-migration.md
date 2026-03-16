# SyscallNo / SyscallErrorCode Newtype Migration

## What changed

`SyscallNo` and `SyscallErrorCode` are now newtype structs with associated constants
instead of `#[repr(i32)]` enums.

```rust
// Before
pub enum SyscallNo { kSHA256 = 0, kGetMsgSender = 1, ... }
pub enum SyscallErrorCode { kSyscallErrorCodeNone = 0, ... }

// After
pub struct SyscallNo(pub u64);
impl SyscallNo { pub const SHA256: Self = Self(0); ... }

pub struct SyscallErrorCode(pub i32);
impl SyscallErrorCode { pub const NONE: Self = Self(0); ... }
```

## Files already updated

- `groundhog-xdr-macros/src/syscalls.rs` — proc macro generates struct + associated constants;
  `generate_syscall_uninit_decl` references updated from `kFoo` to `Foo`
- `groundhog-xdr/src/syscall_error.rs` — replaced enum with struct + constants
  (**NOTE**: this file is generated from `protocol/syscall_error.x` via xdrc —
  the hand-edit shows the target output but will be overwritten on next codegen run;
  xdrc itself needs to be updated to emit the newtype pattern for this type)
- `groundhog-xdr/src/syscalls.rs` — `SysErr` methods updated for new `SyscallErrorCode` constants,
  `syscall_wrapper` uses `.0` instead of `as u64`

## Downstream changes needed

### `groundhog-trace/src/transaction_trace.rs`

`SyscallTraceBody` enum uses `SyscallNo::kFoo as i32` as discriminant values.
These need to change to `SyscallNo::Foo.0 as i32`:

```rust
// Before
kSHA256 { sha256: SyscallSHA256Trace } = SyscallNo::kSHA256 as i32,

// After
kSHA256 { sha256: SyscallSHA256Trace } = SyscallNo::SHA256.0 as i32,
```

### `groundhog_xdr_syscall_tests/tests/test_primitives.rs`

Test assertions compare against `SyscallNo::kFoo as u64`. Change to `.0`:

```rust
// Before
assert_eq!(invocations[0].syscall_no, SyscallNo::kGetCalldataLen as u64);

// After
assert_eq!(invocations[0].syscall_no, SyscallNo::GetCalldataLen.0);
```

### Any external consumers

Anywhere referencing:
- `SyscallNo::kFoo` → `SyscallNo::Foo`
- `SyscallNo::kFoo as u64` → `SyscallNo::Foo.0`
- `SyscallNo::kFoo as i32` → `SyscallNo::Foo.0 as i32`
- `SyscallErrorCode::kSyscallErrorCodeFoo` → `SyscallErrorCode::FOO`
- `SyscallErrorCode::kFoo as i32` → `SyscallErrorCode::FOO.0`
- `TryFrom<i32> for SyscallErrorCode` — removed, no longer needed
  (construct directly with `SyscallErrorCode(value)`)

### Naming reference

| Old SyscallErrorCode | New |
|---|---|
| `kSyscallErrorCodeNone` | `NONE` |
| `kSyscallErrorCodeFault` | `FAULT` |
| `kSyscallErrorCodeNoSyscall` | `NO_SYSCALL` |
| `kSyscallErrorCodeIsRootTransaction` | `IS_ROOT_TRANSACTION` |
| `kSyscallErrorCodeContractAlreadyDeployed` | `CONTRACT_ALREADY_DEPLOYED` |
| `kSyscallErrorCodeNoReturnBufferAvailable` | `NO_RETURN_BUFFER_AVAILABLE` |
| `kSyscallErrorCodeInvalidSyscallInput` | `INVALID_SYSCALL_INPUT` |
| `kSyscallErrorCodeStorageKeyNotExist` | `STORAGE_KEY_NOT_EXIST` |
| `kSyscallErrorCodeNoRetrocausalQueueEntry` | `NO_RETROCAUSAL_QUEUE_ENTRY` |
| `kSyscallErrorCodeDeterministicWasm` | `DETERMINISTIC_WASM` |
| `kSyscallErrorCodeFrameReverted` | `FRAME_REVERTED` |

SyscallNo constants drop the `k` prefix and use the YAML name directly:
`kSHA256` → `SHA256`, `kGetMsgSender` → `GetMsgSender`, etc.
