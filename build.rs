// based on https://rust-lang-nursery.github.io/rust-bindgen/


fn main() {
//    println!("cargo:rustc-link-search=../pugl-rs/pugl/build");
//    println!("cargo:rustc-link-lib=static=pugl_x11");
//    println!("cargo:rustc-link-lib=static=pugl_x11_cairo");
    println!("cargo:rustc-flags=-l cairo -l GLU -l GL -lX11 -lXext -lXrandr")
}
