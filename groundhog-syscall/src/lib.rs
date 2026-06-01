#![no_std]
#![allow(non_snake_case)]

// Guard against building a syscall dispatch path on a target that has none:
// wasm uses the `marmot` host import, riscv32 uses `ecall`, and the mock uses a
// native symbol. Any other target (and docsrs, which type-checks the wasm
// import stub) has no valid path.
#[cfg(all(
    not(feature = "syscalls_mock"),
    not(target_family = "wasm"),
    not(target_arch = "riscv32"),
    not(docsrs)
))]
compile_error!(
    "no syscall dispatch path for this target (expected wasm, riscv32, or syscalls_mock)"
);
