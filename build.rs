extern crate cmake;

fn main() {
    let dst = cmake::build("lib/raylib");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    
    // Borrowed the list for Windows from raylib-rs
    // @see https://github.com/deltaphc/raylib-rs/blob/master/raylib-sys/build.rs#
    println!("cargo:rustc-link-lib=dylib=winmm");
    println!("cargo:rustc-link-lib=dylib=gdi32");
    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-lib=dylib=shell32");
    
    println!("cargo:rustc-link-lib=static=raylib");
}
