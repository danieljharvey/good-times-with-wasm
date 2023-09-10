use crate::types::expr::{Expr, Prim};
use nom::Finish;
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

pub fn expr_to_wasm<Ann>(expr: Expr<Ann>) -> Vec<u8> {
    let mut module = Module::new();

    // Encode the type section.
    let mut types = TypeSection::new();
    let params = vec![];
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
    // export function 0 as 'main'
    exports.export("main", ExportKind::Func, 0);
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

pub fn expr_to_function<Ann>(expr: Expr<Ann>) -> wasm_encoder::Function {
    let locals = vec![];
    let mut f = Function::new(locals);

    expr_to_instructions(&mut f, expr);

    f.instruction(&Instruction::End);

    f
}

fn expr_to_instructions<Ann>(
    f: &mut wasm_encoder::Function,
    expr: Expr<Ann>,
) -> &mut wasm_encoder::Function {
    match expr {
        Expr::EPrim { prim, .. } => f.instruction(&prim_to_const(prim.clone())),
        Expr::EIf {
            pred_expr,
            then_expr,
            else_expr,
            ..
        } => {
            expr_to_instructions(f, *then_expr);
            expr_to_instructions(f, *else_expr);
            expr_to_instructions(f, *pred_expr);
            f.instruction(&Instruction::Select)
        }
        _ => f,
    }
}

fn prim_to_const(prim: Prim) -> Instruction<'static> {
    match prim {
        Prim::PInt { int } => Instruction::I32Const(int),
        Prim::PBool { bool: true } => Instruction::I32Const(1),
        Prim::PBool { bool: false } => Instruction::I32Const(0),
    }
}

#[test]
fn test_run_wasm_eq_from_ast() {
    // 100 == 1
    let (_, input) = crate::parser::parse_expr::parse_my_expr("if True then 42 else 41")
        .finish()
        .unwrap();
    let wasm = expr_to_wasm(input);

    let result = super::run_wasm::run_wasm_from_ast(wasm).unwrap();
    assert_eq!(result, 42)
}
