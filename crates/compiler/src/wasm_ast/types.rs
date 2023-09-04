pub enum Number {
    I32(i32),
}

pub enum Value {
    Const(Number),
    BinaryOp {
        op: BinaryOp,
        left: Box<Value>,
        right: Box<Value>,
    },
}

pub enum Type {
    Int32,
    Int64,
}

pub enum BinaryOp {
    Equal(Type),
}

pub struct WasmModule {
    pub main: Value,
}
