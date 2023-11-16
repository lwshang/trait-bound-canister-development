include!(concat!(env!("OUT_DIR"), "/counter.rs"));

#[derive(Default)]
struct Canister {
    counter: u64,
}

impl Counter for Canister {
    async fn inc(&mut self) {
        self.counter += 1;
    }

    fn read(&self) -> u64 {
        self.counter
    }

    fn canister_init(&mut self, arg0: Option<u64>) {
        self.counter = arg0.unwrap_or(0);
    }

    fn canister_pre_upgrade(&mut self) {
        ic_cdk::storage::stable_save((self.counter,)).unwrap();
    }

    fn canister_post_upgrade(&mut self, arg0: Option<u64>) {
        match arg0 {
            Some(new_start) => self.counter = new_start,
            None => {
                let (previous,): (u64,) = ic_cdk::storage::stable_restore().unwrap();
                self.counter = previous;
            }
        }
    }
}
