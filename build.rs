use std::env;

fn main() {
    //println!("cargo:rustc-link-lib=dylib=FRC_NetworkCommunication");

    let path = env::current_dir().unwrap();
    println!("cargo:rustc-link-search=native={}/lib", path.display());
}
