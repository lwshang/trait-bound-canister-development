use std::path::Path;

static BINDING: &str = r###"trait Counter {
    /// update method: `inc`
    async fn inc(&mut self);
    /// query method: `read`
    fn read(&self) -> u64;
    /// lifecycle method: `canister_init`
    fn canister_init(&mut self, arg0: Option<u64>);
    /// lifecycle method: `canister_pre_upgrade`
    fn canister_pre_upgrade(&mut self);
    /// lifecycle method: `canister_post_upgrade`
    fn canister_post_upgrade(&mut self, arg0: Option<u64>);
}

#[export_name = "canister_update inc"]
fn __canister_method_inc() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let _result = CANISTER.lock().unwrap().inc().await;
        ic_cdk::api::call::reply(())
    });
}

#[export_name = "canister_query read"]
fn __canister_method_read() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let result = CANISTER.lock().unwrap().read();
        ic_cdk::api::call::reply((result,))
    });
}

#[export_name = "canister_init"]
fn __canister_init() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (arg0,) = ic_cdk::api::call::arg_data();
        let _result = CANISTER.lock().unwrap().canister_init(arg0);
    });
}

#[export_name = "canister_pre_upgrade"]
fn __canister_pre_upgrade() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let _result = CANISTER.lock().unwrap().canister_pre_upgrade();
    });
}

#[export_name = "canister_post_upgrade"]
fn __canister_post_upgrade() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let (arg0,) = ic_cdk::api::call::arg_data();
        let _result = CANISTER.lock().unwrap().canister_post_upgrade(arg0);
    });
}

static CANISTER: once_cell::sync::Lazy<std::sync::Mutex<Canister>> =
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(Default::default()));
"###;

fn main() {
    println!("cargo:rerun-if-changed=../counter.did");
    println!("cargo:rerun-if-changed=build.rs");
    // ic_cdk_bindgen::ProviderConfig::new()
    //   .async_methods(&["inc"])
    //   .init()
    //   .pre_upgrade()
    //   .post_upgrade()
    //   .generate("counter.did")
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("counter.rs");
    std::fs::write(path, BINDING).unwrap();
}
