fn main() {
    let dest = cmake::Config::new("cpuinfo")
	.define("CPUINFO_LIBRARY_TYPE", "static")
	.define("CPUINFO_RUNTIME_TYPE", "static")
        .define("CPUINFO_BUILD_TOOLS", "OFF")
        .define("CPUINFO_BUILD_UNIT_TESTS", "OFF")
        .define("CPUINFO_BUILD_MOCK_TESTS", "OFF")
        .define("CPUINFO_BUILD_BENCHMARKS", "OFF")
        .define("CPUINFO_LOG_LEVEL", "none")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dest.display());
    println!("cargo:rustc-link-lib=static=cpuinfo");
    println!("cargo:rustc-link-lib=static=clog");

    let bindings = bindgen::Builder::default()
        .header(&format!("{}/include/cpuinfo.h", dest.display()))
        .rustified_enum("cpuinfo_(vendor|uarch)")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
