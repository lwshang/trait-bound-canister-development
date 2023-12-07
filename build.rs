use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=../counter.did");
    println!("cargo:rerun-if-changed=build.rs");
    // ic_cdk_bindgen::ProviderConfig::new()
    //   .async_methods(&["inc"])
    //   .lifecycle_methods(&["init", "pre_upgrade", "post_upgrade"])
    //   .generate("counter.did")
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("counter.rs");
    std::fs::copy("counter.rs", path).unwrap();
}
