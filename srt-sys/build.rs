use std::env;
use std::path::PathBuf;

fn main() {
    let libs = system_deps::Config::new().probe().unwrap();
    let headers = libs.get("srt").unwrap().include_paths.clone();

    let mut builder = bindgen::builder()
        .header("data/include.h")
        .size_t_is_usize(true)
        .whitelist_function("srt_.*")
        .whitelist_type("SRT.*")
        .whitelist_var("SRT.*")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts);

    for header in headers {
        builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    builder
        .generate()
        .unwrap()
        .write_to_file(out_path.join("srt.rs"))
        .unwrap();
}
