use std::io::Write;
use std::process::Command;
use wasm_encoder::{
    CodeSection, Export, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

pub fn output_wasm() -> Vec<u8> {
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
    exports.export("f", Export::Function(0));
    module.section(&exports);

    // Encode the code section.
    let mut codes = CodeSection::new();
    let locals = vec![];
    let mut f = Function::new(locals);
    f.instruction(&Instruction::LocalGet(0));
    f.instruction(&Instruction::LocalGet(1));
    f.instruction(&Instruction::I32Add);
    f.instruction(&Instruction::End);
    codes.function(&f);
    module.section(&codes);

    // Extract the encoded Wasm bytes for this module.
    let wasm_bytes = module.finish();

    wasm_bytes
}

fn run_wasm(wasm_bytes: Vec<u8>) -> std::io::Result<()> {
    let mut file = std::fs::File::create("test.wasm")?;
    file.write_all(&wasm_bytes)?;
    Ok(())
}

#[test]
fn test_wasm() -> std::io::Result<()> {
    let wasm_bytes = output_wasm();
    run_wasm(wasm_bytes)
}
