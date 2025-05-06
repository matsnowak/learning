use std::panic;

use crate::{
    bytecode::ByteCode,
    lex::{Lex, Token},
    value::Value,
};
#[derive(Debug)]
pub struct ParseProto {
    pub constants: Vec<Value>,
    pub bytecodes: Vec<ByteCode>,
}
pub(crate) fn load(input: std::fs::File) -> ParseProto {
    let mut constants = Vec::new();
    let mut bytecodes = Vec::new();
    let mut lex = Lex::new(input);

    loop {
        match lex.next() {
            Token::Name(name) => {
                constants.push(Value::String(name));
                bytecodes.push(ByteCode::GetGlobal(0, (constants.len() - 1) as u8));

                if let Token::String(s) = lex.next() {
                    constants.push(Value::String(s));
                    bytecodes.push(ByteCode::LoadConst(1, (constants.len() - 1) as u8));
                    bytecodes.push(ByteCode::Call(0, 1));
                } else {
                    panic!("expected string");
                }
            }
            Token::Eos => break,
            t => panic!("unexpected token: {t:?}"),
        }
    }
    dbg!(&constants);
    dbg!(&bytecodes);
    ParseProto {
        constants,
        bytecodes,
    }
}
