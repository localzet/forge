use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    PushInt(i64),
    PushString(String),
    LoadLocal(String),
    StoreLocal(String),
    Call(String),
    Return,
    Print,
    Pop,
}

#[derive(Debug)]
pub struct FunctionBytecode {
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub params: Vec<String>,
}

pub struct Compiler {
    pub functions: HashMap<String, FunctionBytecode>,
}

impl Compiler {
    pub fn new() -> Self {
        Self { functions: HashMap::new() }
    }

    pub fn compile(&mut self, program: Program) {
        for func in program.functions {
            let mut instrs = Vec::new();
            for stmt in func.body {
                self.compile_statement(&mut instrs, &stmt);
            }
            // ensure end with Return
            instrs.push(Instruction::Return);
            let params = func.parameters.into_iter().map(|(n, _)| n).collect();
            let fb = FunctionBytecode { name: func.name.clone(), instructions: instrs, params };
            self.functions.insert(func.name, fb);
        }
    }

    fn compile_statement(&self, instrs: &mut Vec<Instruction>, stmt: &Statement) {
        match stmt {
            Statement::Let { name, value, .. } => {
                self.compile_expression(instrs, value);
                instrs.push(Instruction::StoreLocal(name.clone()));
            }
            Statement::Return(opt) => {
                if let Some(e) = opt {
                    self.compile_expression(instrs, e);
                }
                instrs.push(Instruction::Return);
            }
            Statement::Expression(e) => {
                self.compile_expression(instrs, e);
                instrs.push(Instruction::Pop);
            }
        }
    }

    fn compile_expression(&self, instrs: &mut Vec<Instruction>, expr: &Expression) {
        match expr {
            Expression::IntegerLiteral(i) => instrs.push(Instruction::PushInt(*i)),
            Expression::StringLiteral(s) => instrs.push(Instruction::PushString(s.clone())),
            Expression::BooleanLiteral(b) => instrs.push(Instruction::PushInt(if *b {1} else {0})),
            Expression::Variable(name) => instrs.push(Instruction::LoadLocal(name.clone())),
            Expression::ArrayLiteral(_) => unimplemented!("arrays not compiled yet"),
            Expression::ArrayAccess(_, _) => unimplemented!("array access not compiled yet"),
            Expression::FunctionCall(name, args) => {
                for a in args {
                    self.compile_expression(instrs, a);
                }
                instrs.push(Instruction::Call(name.clone()));
            }
        }
    }
}
