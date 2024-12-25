// TODO: Only enable for "CSharp" feature
fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/ffi/mod.rs")
        .input_extern_file("src/error/mod.rs")
        .input_extern_file("src/ffi/log.rs")
        .csharp_dll_name("tremble")
        .generate_csharp_file("dotnet/NativeMethods.g.cs")
        .unwrap();
}
