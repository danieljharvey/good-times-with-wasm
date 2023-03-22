use wasm_encoder::{
    CodeSection, Export, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use super::runwasm::run_wasm;

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
    exports.export("main", Export::Function(0));
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

#[test]
fn run_sample_wasm() {
    match run_wasm(output_wasm()) {
        Ok(a) => assert_eq!(a, 42),
        Err(err) => {
            println!("{}", err);
            assert_eq!(true, false)
        }
    }
}
