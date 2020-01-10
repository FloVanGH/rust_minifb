use std::env;
extern crate cc;

#[cfg(feature = "utouch")]
extern crate cpp_build;

#[cfg(feature = "utouch")]
use std::process::Command;

#[cfg(feature = "utouch")]
fn qmake_query(var: &str) -> String {
    String::from_utf8(
        Command::new("qmake")
            .args(&["-query", var])
            .output()
            .expect("Failed to execute qmake. Make sure 'qmake' is in your path")
            .stdout,
    )
    .expect("UTF-8 conversion failed")
}

#[cfg(feature = "utouch")]
fn utouch() {
    let qt_include_path = qmake_query("QT_INSTALL_HEADERS");
    let qt_library_path = qmake_query("QT_INSTALL_LIBS");
    cpp_build::Config::new()
        .include(qt_include_path.trim())
        .build("src/os/utouch/mod.rs");

    let macos_lib_search = if cfg!(target_os = "macos") {
        "=framework"
    } else {
        ""
    };
    let macos_lib_framework = if cfg!(target_os = "macos") { "" } else { "5" };
    println!(
        "cargo:rustc-link-search{}={}",
        macos_lib_search,
        qt_library_path.trim()
    );
    println!(
        "cargo:rustc-link-lib{}=Qt{}Widgets",
        macos_lib_search, macos_lib_framework
    );
    println!(
        "cargo:rustc-link-lib{}=Qt{}Gui",
        macos_lib_search, macos_lib_framework
    );
    println!(
        "cargo:rustc-link-lib{}=Qt{}Core",
        macos_lib_search, macos_lib_framework
    );
    println!(
        "cargo:rustc-link-lib{}=Qt{}Quick",
        macos_lib_search, macos_lib_framework
    );
    println!(
        "cargo:rustc-link-lib{}=Qt{}Qml",
        macos_lib_search, macos_lib_framework
    );
    println!(
        "cargo:rustc-link-lib{}=Qt{}QuickControls2",
        macos_lib_search, macos_lib_framework
    );
}

fn main() {
    #[cfg(feature = "utouch")]
    utouch();

    let env = env::var("TARGET").unwrap();
    if env.contains("darwin") {
        cc::Build::new()
            .flag("-mmacosx-version-min=10.10")
            .file("src/native/macosx/MacMiniFB.m")
            .file("src/native/macosx/OSXWindow.m")
            .file("src/native/macosx/OSXWindowFrameView.m")
            .compile("libminifb_native.a");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
    } else if !env.contains("windows") {
        // build scalar on non-windows and non-mac
        cc::Build::new()
            .file("src/native/unix/scalar.cpp")
            .opt_level(3) // always build with opts for scaler so it's fast in debug also
            .compile("libscalar.a")
    }
}
