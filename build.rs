use std::env;

// TODO: Only enable for "CSharp" feature
fn main() {
    if env::var_os("CARGO_FEATURE_DISABLE_GEN").is_some() {
        return;
    }

    csbindgen::Builder::default()
        .input_extern_file("src/ffi/mod.rs")
        .input_extern_file("src/error/expanded.g.rs")
        .input_extern_file("src/ffi/log.rs")
        .csharp_dll_name("tremble")
        .generate_csharp_file("dotnet/NativeMethods.g.cs")
        .unwrap();
}
