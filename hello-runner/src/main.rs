cargo_component_bindings::generate!();

use bindings::hello_world;

fn main() {
    println!("Hello runner!");
    println!("{}", hello_world());
}
