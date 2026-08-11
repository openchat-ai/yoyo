//! Debug: print wasmtime validation error
use wasmtime::Engine;

fn main() {
    let bytes = std::fs::read("f:\\yoyo\\yoyo-rust\\build\\verify\\wasm.out").unwrap();
    println!("wasm bytes: {} bytes, first 4: {:02X} {:02X} {:02X} {:02X}", 
        bytes.len(), bytes[0], bytes[1], bytes[2], bytes[3]);
    let engine = Engine::default();
    match wasmtime::Module::new(&engine, &bytes) {
        Ok(module) => {
            println!("Module::new OK");
            let mut store = wasmtime::Store::new(&engine, ());
            match wasmtime::Instance::new(&mut store, &module, &[]) {
                Ok(_) => println!("Instance::new OK"),
                Err(e) => println!("Instance::new ERROR: {e}"),
            }
        }
        Err(e) => println!("Module::new ERROR: {e}"),
    }
}