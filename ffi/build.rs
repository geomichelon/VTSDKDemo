fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=../core/src");
    // Try to generate C header. If cbindgen is not available, just warn.
    let out = std::path::Path::new("include");
    let _ = std::fs::create_dir_all(out);
    match cbindgen::Builder::new()
        .with_crate(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .with_language(cbindgen::Language::C)
        .generate()
    {
        Ok(bindings) => {
            let header = out.join("vt_sdk.h");
            bindings.write_to_file(header);
        }
        Err(err) => {
            println!("cargo:warning=cbindgen failed: {err}");
        }
    }
}

