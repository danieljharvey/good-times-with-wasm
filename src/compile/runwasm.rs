use anyhow::Result;
use wasmtime::*;

fn run_sample_wasm() -> Result<i32> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    let wat = r#"
        (module
            (import "host" "id" (func $host_id (param i32) (result i32)))

            (func (export "hello") (param $0 i32) (result i32)
                i32.const 1
                get_local $0
                i32.add
                call $host_id)
        )
    "#;
    let module = Module::new(&engine, wat)?;

    // Create a `Linker` which will be later used to instantiate this module.
    // Host functionality is defined by name within the `Linker`.
    let mut linker = Linker::new(&engine);

    // example external function we might want to create
    linker.func_wrap("host", "id", |caller: Caller<'_, u32>, param: i32| -> i32 {
        println!("Got {} from WebAssembly", param);
        println!("my host state is: {}", caller.data());
        let internal_state: i32 = (*(caller.data())).try_into().unwrap();
        param + internal_state
    })?;

    // All wasm objects operate within the context of a "store". Each
    // `Store` has a type parameter to store host-specific data, which in
    // this case we're using `4` for.
    let mut store = Store::new(&engine, 4);
    let instance = linker.instantiate(&mut store, &module)?;
    let hello = instance.get_typed_func::<i32, i32>(&mut store, "hello")?;

    // And finally we can call the wasm!
    hello.call(&mut store, 37)
}

// all wasm we pass in must take two i32 args and return an `i32`
// we run the `main` function
pub fn run_wasm(wasm_bytes: Vec<u8>) -> Result<i32> {
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
    let main_fn = instance.get_typed_func::<(i32, i32), i32>(&mut store, "main")?;

    // And finally we can call the wasm!
    main_fn.call(&mut store, (20, 22))
}

#[test]
fn test_run_wasm() {
    match run_sample_wasm() {
        Ok(result) => assert_eq!(result, 42),
        Err(err) => {
            println!("{}", err);
            assert_eq!(true, false)
        }
    }
}
