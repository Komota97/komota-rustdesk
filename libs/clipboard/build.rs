fn build_c_impl() {
    let mut build = cc::Build::new();

    build.file("src/windows/wf_cliprdr.c");

    {
        build.flag_if_supported("-Wno-c++0x-extensions");
        build.flag_if_supported("-Wno-return-type-c-linkage");
        build.flag_if_supported("-Wno-invalid-offsetof");
        build.flag_if_supported("-Wno-unused-parameter");

        if build.get_compiler().is_like_msvc() {
            build.define("WIN32", "");
            // build.define("_AMD64_", "");
            build.flag("-Z7");
            build.flag("-GR-");
            // build.flag("-std:c++11");
        } else {
            build.flag("-fPIC");
            // build.flag("-std=c++11");
            // build.flag("-include");
            // build.flag(&confdefs_path.to_string_lossy());
        }

        build.compile("mycliprdr");
    }

    println!("cargo:rerun-if-changed=src/windows/wf_cliprdr.c");
}

fn main() {
    // Komota: `#[cfg(target_os = "windows")]` here would reflect the HOST running
    // this build script (build scripts always compile for the host), not the
    // actual target, breaking cross-compilation from a non-Windows host. Use
    // CARGO_CFG_TARGET_OS instead (same fix applied to scrap's and the root
    // crate's build.rs for the identical bug).
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        build_c_impl();
    }
}
