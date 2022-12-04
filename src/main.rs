use my_proc_macros::AddMyField;

extern crate my_lib;
use my_lib::print_sum_fields;

// #[derive(AddMyField)]
struct Point2 {
    x: f64,
    y: f64,
}

fn main() {
    println!("Hello, world!");

    // Example of using imported library
    print_sum_fields();

    let point = Point2 { x: 5.0, y: 6.0 };
}
