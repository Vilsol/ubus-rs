use std::env;
use std::path::{PathBuf};

fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=ubus");
    println!("cargo:rustc-link-lib=blobmsg_json");

    let bindings = bindgen::Builder::default()
        .header("/usr/local/include/libubus.h")
        .header("/usr/local/include/libubox/blobmsg_json.h")
        .allowlist_function("ubus.*")
        .allowlist_type("ubus.*")
        .allowlist_var("ubus.*")
        .allowlist_function("blob.*")
        .allowlist_var("blob.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs");
}