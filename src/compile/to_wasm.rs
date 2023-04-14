use wasm_encoder::{
    CodeSection, Export, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use super::runwasm::run_wasm;
use crate::types::expr::{Expr, Prim};

pub fn expr_to_wasm<Ann>(expr: Expr<Ann>) -> Vec<u8> {
    let mut module = Module::new();

    // Encode the type section.
    let mut types = TypeSection::new();
    let params = vec![ValType::I32, ValType::I32];
    let results = vec![ValType::I32];
    types.function(params, results);
    module.section(&types);

    // Encode the function section.
    let mut functions = FunctionSection::new();
    let type_index = 0;
    functions.function(type_index);
    module.section(&functions);

    // Encode the export section.
    let mut exports = ExportSection::new();
    exports.export("main", Export::Function(0));
    module.section(&exports);

    // Encode the code section.
    let mut codes = CodeSection::new();
    let f = expr_to_function(expr);

    codes.function(&f);
    module.section(&codes);

    // Extract the encoded Wasm bytes for this module.
    let wasm_bytes = module.finish();

    wasm_bytes
}

fn expr_to_function<Ann>(expr: Expr<Ann>) -> wasm_encoder::Function {
    let locals = vec![];
    let mut f = Function::new(locals);

    match expr {
        Expr::EPrim { prim, .. } => f.instruction(&prim_to_const(prim.clone())),
        _ => &mut f,
    };
    f.instruction(&Instruction::End);
    f
}

fn prim_to_const(prim: Prim) -> Instruction<'static> {
    match prim {
        Prim::PInt { int } => Instruction::I32Const(int),
        Prim::PBool { bool: true } => Instruction::I32Const(1),
        Prim::PBool { bool: false } => Instruction::I32Const(0),
    }
}

#[test]
fn run_sample_wasm() {
    let expr = Expr::EIf {
        ann: (),
        pred_expr: Box::new(Expr::EPrim {
            ann: (),
            prim: Prim::PBool { bool: false },
        }),
        then_expr: Box::new(Expr::EPrim {
            ann: (),
            prim: Prim::PInt { int: 21 },
        }),
        else_expr: Box::new(Expr::EPrim {
            ann: (),
            prim: Prim::PInt { int: 42 },
        }),
    };

    match run_wasm(expr_to_wasm(expr)) {
        Ok(a) => assert_eq!(a, 42),
        Err(err) => {
            println!("{}", err);
            assert_eq!(true, false)
        }
    }
}