use crate::ast::pretty::Pretty;
use crate::ast::types::{BinaryOp, Number, Type, Value, WasmModule};
use anyhow::Result;
use wasmtime::*;

// all wasm we pass in must take two i32 args and return an `i32`
// we run the `main` function
pub fn run_wasm_from_string(wat_string: String) -> Result<i32> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    let module = Module::new(&engine, wat_string)?;

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
fn test_run_wasm_from_string() {
    // 100 == 1
    let input = WasmModule {
        main: Value::BinaryOp {
            op: BinaryOp::Equal(Type::Int32),
            left: Box::new(Value::Const(Number::I32(100))),
            right: Box::new(Value::Const(Number::I32(1))),
        },
    };
    let mut wat_string = "".to_string();
    input.pretty(&mut wat_string);

    let result = run_wasm_from_string(wat_string).unwrap();
    assert_eq!(result, 0)
}
