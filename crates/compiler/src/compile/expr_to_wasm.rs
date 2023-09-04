use crate::types::expr::{Expr, Prim};
use crate::wasm_ast::types::{Number, Value, WasmModule};

pub fn expr_to_wasm_module<Ann>(expr: Expr<Ann>) -> WasmModule {
    WasmModule {
        main: expr_to_wasm_value(expr),
    }
}

fn expr_to_wasm_value<Ann>(expr: Expr<Ann>) -> Value {
    match expr {
        Expr::EPrim { prim, .. } => match prim {
            Prim::PBool { bool } => {
                let num = if bool { 1 } else { 0 };
                Value::Const(Number::I32(num))
            }
            Prim::PInt { int } => Value::Const(Number::I32(int)),
        },
        _ => todo!("make it work"),
    }
}
