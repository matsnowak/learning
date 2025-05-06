use std::collections::HashMap;

use crate::{bytecode::ByteCode, value::Value};

pub struct ExeState {
    globals: HashMap<String, Value>,
    stack: Vec<Value>,
}
impl ExeState {
    pub(crate) fn new() -> Self {
        let mut globals = HashMap::new();
        globals.insert(String::from("print"), Value::Function(lib_print));
        Self {
            globals,
            stack: Vec::new(),
        }
    }

    pub(crate) fn execute(&mut self, proto: crate::parse::ParseProto) {
        for code in proto.bytecodes.iter() {
            match *code {
                ByteCode::GetGlobal(dst, name) => {
                    let name = &proto.constants[name as usize];
                    if let Value::String(key) = name {
                        let v = self.globals.get(key).unwrap_or(&Value::Nil).clone();
                        self.set_stack(dst, v);
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
                ByteCode::LoadConst(dst, c) => {
                    let v = proto.constants[c as usize].clone();
                    self.set_stack(dst, v);
                }
                ByteCode::Call(func, _) => {
                    let func = &self.stack[func as usize];
                    if let Value::Function(f) = func {
                        f(self);
                    } else {
                        panic!("invalid function {func:?}");
                    }
                }
            }
        }
    }

    fn set_stack(&mut self, dst: u8, v: Value) {
        self.stack[dst as usize] = v;
    }
}

fn lib_print(state: &mut ExeState) -> i32 {
    println!("{:?}", state.stack[1]);
    0
}
