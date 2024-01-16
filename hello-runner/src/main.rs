cargo_component_bindings::generate!();

use bindings::component::hello_wasm::hello::hello_world;

fn main() {
    println!("Hello runner!");
    println!("{}", hello_world());
}
