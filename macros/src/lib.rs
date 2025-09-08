mod syscalls;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_syscall_decls(input: TokenStream) -> TokenStream {
    syscalls::generate_syscall_decls(input)
}

#[proc_macro]
pub fn generate_syscall_numbers(input: TokenStream) -> TokenStream {
    syscalls::generate_syscall_numbers(input)
}
