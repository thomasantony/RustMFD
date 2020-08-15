fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/wrapper.cc")
        .file("src/MFDTemplate.cpp")
        .include(".")
        .include("D:\\code\\Orbitersdk\\include\\")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/wrapper.cc");
    println!("cargo:rustc-link-lib=Orbiter");
    println!("cargo:rustc-link-lib=Orbitersdk");
    println!("cargo:rustc-link-search=D:\\code\\Orbitersdk\\lib");
}