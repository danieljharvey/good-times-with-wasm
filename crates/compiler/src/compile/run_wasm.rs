use crate::wasm_ast::types::{BinaryOp, Number, Type, Value, WasmModule};
use anyhow::Result;
use wasmtime::*;

use crate::wasm_ast::pretty::Pretty;

// all wasm we pass in must take no args and return an `i32`
// we run the `main` function
#[cfg(test)]
pub fn run_wasm_from_string(wat_string: String) -> Result<i32> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    println!("{}", wat_string);
    let module = Module::new(&engine, wat_string)?;

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

#[test]
fn test_run_wasm_eq_from_ast() {
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

#[test]
fn test_run_wasm_if_from_ast() {
    // if 1 then 42 else 41
    let input = WasmModule {
        main: Value::Select {
            pred_expr: Box::new(Value::Const(Number::I32(1))),
            then_expr: Box::new(Value::Const(Number::I32(42))),
            else_expr: Box::new(Value::Const(Number::I32(41))),
        },
    };
    let mut wat_string = "".to_string();
    input.pretty(&mut wat_string);

    let result = run_wasm_from_string(wat_string).unwrap();
    assert_eq!(result, 42)
}
