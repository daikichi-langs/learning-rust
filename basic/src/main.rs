// mod vars;
// mod stack_heap;
// mod ownership;
// mod generics;
// mod lifetime;
// mod structs;
// mod enums;
// mod traits;
mod debug;
mod error_handling;
mod unit_test;

extern crate lib_basic;
fn main() {
    // println!("Hello, world!");
    // vars::sub_a::run();
    // vars::run();
    // stack_heap::run();
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    error_handling::run();
    lib_basic::print_random_number();
    debug::run();
}
