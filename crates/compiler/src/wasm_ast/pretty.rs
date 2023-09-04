use crate::wasm_ast::types;

pub trait Pretty {
    // Associated function signature; `Self` refers to the implementor type.
    fn pretty(&self, str: &mut String) -> ();
}

impl Pretty for types::WasmModule {
    fn pretty(&self, str: &mut String) {
        match self {
            types::WasmModule { main } => {
                str.push_str("(module ");
                str.push_str("(func $main (param $lhs i32) (param $rhs i32) (result i32) ");
                main.pretty(str);
                str.push_str(")"); // end func
                str.push_str("(export \"main\" (func $main))");
                str.push_str(")") // end module
            }
        }
    }
}

impl Pretty for types::Number {
    fn pretty(&self, str: &mut String) {
        match self {
            types::Number::I32(i) => str.push_str(&i.to_string()),
        }
    }
}

impl Pretty for types::Value {
    fn pretty(&self, str: &mut String) {
        match self {
            types::Value::Const(types::Number::I32(i)) => {
                str.push_str("i32.const ");
                str.push_str(&i.to_string())
            }
            types::Value::BinaryOp { op, left, right } => {
                left.pretty(str);
                str.push_str("\n");
                right.pretty(str);
                str.push_str("\n");
                op.pretty(str)
            }
        }
    }
}

impl Pretty for types::BinaryOp {
    fn pretty(&self, str: &mut String) {
        match self {
            types::BinaryOp::Equal(ty) => {
                ty.pretty(str);
                str.push_str(".eq")
            }
        }
    }
}

impl Pretty for types::Type {
    fn pretty(&self, str: &mut String) {
        match self {
            types::Type::Int32 => str.push_str("i32"),
            types::Type::Int64 => str.push_str("i64"),
        }
    }
}

#[test]
fn test_pretty_number() {
    let input = types::Number::I32(100);
    let mut initial = "".to_string();
    input.pretty(&mut initial);
    assert_eq!(initial, "100");
}

#[test]
fn test_pretty_const() {
    let input = types::Value::Const(types::Number::I32(100));
    let mut initial = "".to_string();
    input.pretty(&mut initial);
    assert_eq!(initial, "i32.const 100");
}

#[test]
fn test_pretty_equals() {
    let input = types::Value::BinaryOp {
        op: types::BinaryOp::Equal(types::Type::Int32),
        left: Box::new(types::Value::Const(types::Number::I32(100))),
        right: Box::new(types::Value::Const(types::Number::I32(1))),
    };
    let mut initial = "".to_string();
    input.pretty(&mut initial);
    assert_eq!(initial, "i32.const 100\ni32.const 1\ni32.eq");
}
