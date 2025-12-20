use crate::compiler::{FunctionBytecode, Instruction};
use std::collections::HashMap;

pub struct VM {
    functions: HashMap<String, FunctionBytecode>,
    stack: Vec<Value>,
    locals: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Str(String),
    None,
}

impl VM {
    pub fn new(functions: HashMap<String, FunctionBytecode>) -> Self {
        Self { functions, stack: Vec::new(), locals: HashMap::new() }
    }

    pub fn run(&mut self, entry: &str) {
        let fb = self.functions.get(entry).expect("Entry function not found");
        self.execute(&fb.instructions);
    }

    fn execute(&mut self, instrs: &[Instruction]) {
        let mut ip = 0usize;
        while ip < instrs.len() {
            match &instrs[ip] {
                Instruction::PushInt(i) => self.stack.push(Value::Int(*i)),
                Instruction::PushString(s) => self.stack.push(Value::Str(s.clone())),
                Instruction::LoadLocal(name) => {
                    let v = self.locals.get(name).cloned().unwrap_or(Value::None);
                    self.stack.push(v);
                }
                Instruction::StoreLocal(name) => {
                    let v = self.stack.pop().unwrap_or(Value::None);
                    self.locals.insert(name.clone(), v);
                }
                Instruction::Call(name) => {
                    if name == "print" {
                        let v = self.stack.pop().unwrap_or(Value::None);
                        match v {
                            Value::Int(i) => println!("{}", i),
                            Value::Str(s) => println!("{}", s),
                            Value::None => println!("")
                        }
                    } else {
                        // call user function (no args/locals per simple model)
                        if let Some(fb) = self.functions.get(name) {
                            self.execute(&fb.instructions);
                        } else {
                            panic!("Unknown function '{}'", name);
                        }
                    }
                }
                Instruction::Return => {
                    return;
                }
                Instruction::Print => {
                    let v = self.stack.pop().unwrap_or(Value::None);
                    match v {
                        Value::Int(i) => println!("{}", i),
                        Value::Str(s) => println!("{}", s),
                        Value::None => println!("")
                    }
                }
                Instruction::Pop => { self.stack.pop(); }
            }
            ip += 1;
        }
    }
}
