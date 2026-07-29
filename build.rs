fn build_windows() {
    let file = "src/platform/windows.cc";
    let file2 = "src/platform/windows_delete_test_cert.cc";
    let mut build = cc::Build::new();
    build.file(file).file(file2);
    // Komota: needed when cross-compiling via mingw-w64's GCC (never triggers on
    // the officially-supported native MSVC build). Some of mingw's WTS API surface
    // used in this file is version-gated; see the WTSSessionInfoEx compat shim
    // added directly in windows.cc for the parts mingw's headers omit entirely.
    build.define("_WIN32_WINNT", "0x0601");
    build.define("WINVER", "0x0601");
    if !build.get_compiler().is_like_msvc() {
        // GCC/mingw only: this file has a couple of `goto` jumps that skip past a
        // variable's initializer, which MSVC's cl.exe tolerates but GCC rejects
        // under strict conformance. -fpermissive downgrades it to a warning.
        build.flag("-fpermissive");
        // windows_delete_test_cert.cc brace-initializes a `char` array with byte
        // values >127 (fine as data, but GCC's C++11 list-init narrowing check
        // treats it as an error; MSVC doesn't enforce this the same way here).
        build.flag("-Wno-narrowing");
    }
    build.compile("windows");
    println!("cargo:rustc-link-lib=WtsApi32");
    println!("cargo:rerun-if-changed={}", file);
    println!("cargo:rerun-if-changed={}", file2);
}

fn build_mac() {
    let file = "src/platform/macos.mm";
    let mut b = cc::Build::new();
    if let Ok(os_version::OsVersion::MacOS(v)) = os_version::detect() {
        let v = v.version;
        if v.contains("10.14") {
            b.flag("-DNO_InputMonitoringAuthStatus=1");
        }
    }
    b.flag("-std=c++17").file(file).compile("macos");
    println!("cargo:rerun-if-changed={}", file);
}

#[cfg(feature = "inline")]
fn build_manifest() {
    use std::io::Write;
    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res/icon.ico")
            .set_language(winapi::um::winnt::MAKELANGID(
                winapi::um::winnt::LANG_ENGLISH,
                winapi::um::winnt::SUBLANG_ENGLISH_US,
            ))
            .set_manifest_file("res/manifest.xml");
        match res.compile() {
            Err(e) => {
                write!(std::io::stderr(), "{}", e).unwrap();
                std::process::exit(1);
            }
            Ok(_) => {}
        }
    }
}

fn install_android_deps() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os != "android" {
        return;
    }
    let mut target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "x86_64" {
        target_arch = "x64".to_owned();
    } else if target_arch == "x86" {
        target_arch = "x86".to_owned();
    } else if target_arch == "aarch64" {
        target_arch = "arm64".to_owned();
    } else {
        target_arch = "arm".to_owned();
    }
    let target = format!("{}-android", target_arch);
    let vcpkg_root = std::env::var("VCPKG_ROOT").unwrap();
    let mut path: std::path::PathBuf = vcpkg_root.into();
    if let Ok(vcpkg_root) = std::env::var("VCPKG_INSTALLED_ROOT") {
        path = vcpkg_root.into();
    } else {
        path.push("installed");
    }
    path.push(target);
    println!(
        "cargo:rustc-link-search={}",
        path.join("lib").to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=ndk_compat");
    println!("cargo:rustc-link-lib=oboe");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=OpenSLES");
}

fn main() {
    hbb_common::gen_version();
    install_android_deps();
    // NOTE (Komota): `#[cfg(windows)]`/`#[cfg(target_os = "macos")]` on the call sites
    // below would reflect the HOST running this build script (since build scripts
    // always compile for the host), not the actual compilation TARGET -- breaking
    // cross-compilation from a non-Windows/non-macOS host. Use CARGO_CFG_TARGET_OS
    // (already used correctly elsewhere in this file) instead.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "windows" {
        #[cfg(feature = "inline")]
        build_manifest();
        build_windows();
    }
    if target_os == "macos" {
        build_mac();
        println!("cargo:rustc-link-lib=framework=ApplicationServices");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
