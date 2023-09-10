use anyhow::Result;
use wasmtime::*;

// all wasm we pass in must take no args and return an `i32`
// we run the `main` function
#[cfg(test)]
pub fn run_wasm_from_ast(wasm_bytes: Vec<u8>) -> Result<i32> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    let module = Module::new(&engine, wasm_bytes)?;

    // Create a `Linker` which will be later used to instantiate this module.
    // Host functionality is defined by name within the `Linker`.
    let linker = Linker::new(&engine);

    // All wasm objects operate within the context of a "store". Each
    // `Store` has a type parameter to store host-specific data, which in
    // this case we're using `4` for.
    let mut store = Store::new(&engine, 4);
    let instance = linker.instantiate(&mut store, &module)?;
    let main_fn = instance.get_typed_func::<(), i32>(&mut store, "main")?;

    // And finally we can call the wasm!
    main_fn.call(&mut store, ())
}
