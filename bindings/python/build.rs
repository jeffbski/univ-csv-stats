fn main() {
    // This build script is used to configure the linker for PyO3 on macOS.
    // When building a dynamic library that will be loaded by Python, the linker
    // needs to know that some symbols will be provided by the Python interpreter
    // at runtime. On macOS, this is achieved by passing the `-undefined dynamic_lookup`
    // flags to the linker. This tells the linker to not fail on undefined symbols
    // and to resolve them when the library is loaded.
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");
    }
}
