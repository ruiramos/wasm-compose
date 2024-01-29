mod bindings;

use bindings::exports::component::hello_wasm::hello::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}
