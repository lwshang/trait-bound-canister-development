include!(concat!(env!("OUT_DIR"), "/counter.rs"));

struct Canister;

use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<u64> = RefCell::new(0);
}

impl Counter for Canister {
    async fn inc() {
        COUNTER.with_borrow_mut(|c| *c += 1);
    }

    fn read() -> u64 {
        COUNTER.with_borrow(|c| *c)
    }

    fn canister_init(arg0: Option<u64>) {
        // set the counter with start value or default 0
        COUNTER.with_borrow_mut(|c| *c = arg0.unwrap_or(0));
    }

    fn canister_pre_upgrade() {
        // save the counter value into stable memory
        let c = Self::read();
        ic_cdk::storage::stable_save((c,)).unwrap();
    }

    fn canister_post_upgrade(arg0: Option<u64>) {
        match arg0 {
            // set the counter with new start value
            Some(new_start) => COUNTER.with_borrow_mut(|c| *c = new_start),
            // restore the counter value from stable memory
            None => {
                let (previous,): (u64,) = ic_cdk::storage::stable_restore().unwrap();
                COUNTER.with_borrow_mut(|c| *c = previous);
            }
        }
    }
}
