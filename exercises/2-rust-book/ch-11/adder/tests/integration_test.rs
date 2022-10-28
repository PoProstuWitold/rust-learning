// $ cargo test --test integration_test
// use adder;

// #[test]
// fn it_adds_two() {
//     assert_eq!(4, adder::add_two(2));
// }

// We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. 
// Cargo treats the tests directory specially and compiles files 
// in this directory only when we run cargo test
// Submodules in Integration Tests
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     ├── common
//     │   └── mod.rs
//     └── integration_test.rs

use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}