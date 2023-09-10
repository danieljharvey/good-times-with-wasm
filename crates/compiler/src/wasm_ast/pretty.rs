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
                str.push_str("(func $main (result i32) ");
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

fn space(str: &mut String) {
    str.push_str(" ")
}

fn open(str: &mut String) {
    str.push_str("(")
}

fn close(str: &mut String) {
    str.push_str(")")
}

impl Pretty for types::Value {
    fn pretty(&self, str: &mut String) {
        match self {
            types::Value::Const(types::Number::I32(i)) => {
                open(str);
                str.push_str("i32.const ");
                str.push_str(&i.to_string());
                close(str)
            }
            types::Value::BinaryOp { op, left, right } => {
                open(str);
                op.pretty(str);
                space(str);
                left.pretty(str);
                space(str);
                right.pretty(str);
                close(str)
            }
            types::Value::Select {
                pred_expr,
                then_expr,
                else_expr,
            } => {
                open(str);
                str.push_str("select ");
                then_expr.pretty(str);
                space(str);
                else_expr.pretty(str);
                space(str);
                pred_expr.pretty(str);
                close(str)
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
    assert_eq!(initial, "(i32.const 100)");
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
    assert_eq!(initial, "(i32.eq (i32.const 100) (i32.const 1))");
}

#[test]
fn test_pretty_select() {
    let input = types::Value::Select {
        pred_expr: Box::new(types::Value::Const(types::Number::I32(1))),
        then_expr: Box::new(types::Value::Const(types::Number::I32(42))),
        else_expr: Box::new(types::Value::Const(types::Number::I32(41))),
    };

    let mut initial = "".to_string();
    input.pretty(&mut initial);
    assert_eq!(
        initial,
        "(select (i32.const 42) (i32.const 41) (i32.const 1))"
    );
}
