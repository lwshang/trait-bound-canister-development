# Trait-bound Canister Development

This is a complete simulation of the proposing trait-bound canister development.

## Generate the trait

In [`build.rs`](build.rs):

```rust
ic_cdk_bindgen::ProviderConfig::new()
    .async_methods(&["inc"])         // specify the methods to be async
    .init()                          // include the canister_init method
    .pre_upgrade()                   // include the canister_pre_upgrade method
    .post_upgrade()                  // include the canister_post_upgrade method
    .generate("counter.did")         // generate the binding from the specified path of Candid file
```

## The generated code

The generated code is hard coded in this simulation as `BINDING` in [`build.rs`](build.rs).

## Async methods

The `inc` update method in this simulation is `async`.

As of mid-Nov 2023, `async_fn_in_trait` is in beta Rust 1.75. It will be in stable Rust at the end of 2023.
