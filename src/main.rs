use llvm_ir::module::Alignments;
use llvm_ir::module::Endianness::LittleEndian;
use llvm_ir::module::{DataLayout, Module};
use std::collections::hash_set::HashSet;

fn create_module() -> Module {
    let default_data_layout: DataLayout = DataLayout {
        alignments: Alignments::default(),
        alloca_address_space: 0,
        endianness: LittleEndian,
        layout_str: "nice".to_string(),
        mangling: None,
        stack_alignment: None,
        non_integral_ptr_types: HashSet::new(),
        program_address_space: 0,
        native_int_widths: None,
    };

    let my_module: Module = Module {
        name: "dogs".to_string(),
        source_file_name: "dogs.mimsa".to_string(),
        data_layout: default_data_layout,
        target_triple: None,
        functions: vec![],
        global_vars: vec![],
        global_aliases: vec![],
        inline_assembly: "".to_string(),
        types: llvm_ir::types::Types::blank_for_testing(),
    };

    my_module
}

fn main() {
    println!("Hello, ding dongs");
}
