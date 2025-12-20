use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Boolean,
    String,
    Array(Box<Type>),
    Custom(String),
    Unknown,
}

pub struct TypeChecker {
    pub variables: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self { variables: HashMap::new() }
    }

    pub fn check_statement(&mut self, stmt: &Statement) -> Type {
        match stmt {
            Statement::Let { name, type_name, value } => {
                let inferred = self.check_expression(value);
                let t = if let Some(tn) = type_name {
                    let explicit = self.parse_type_name(tn);
                    if explicit != inferred {
                        panic!("Type mismatch for '{}' (expected {:?}, found {:?})", name, explicit, inferred);
                    }
                    explicit
                } else {
                    inferred
                };
                self.variables.insert(name.clone(), t.clone());
                t
            }
            Statement::Return(opt) => {
                if let Some(e) = opt { self.check_expression(e) } else { Type::Unknown }
            }
            Statement::Expression(expr) => self.check_expression(expr),
        }
    }

    pub fn check_expression(&self, expr: &Expression) -> Type {
        match expr {
            Expression::IntegerLiteral(_) => Type::Integer,
            Expression::BooleanLiteral(_) => Type::Boolean,
            Expression::StringLiteral(_) => Type::String,
            Expression::Variable(name) => {
                self.variables.get(name).cloned().unwrap_or(Type::Unknown)
            }
            Expression::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    Type::Array(Box::new(Type::Unknown))
                } else {
                    let (_, first_val) = &elements[0];
                    Type::Array(Box::new(self.check_expression(first_val)))
                }
            }
            Expression::ArrayAccess(_, _) => Type::Unknown,
            Expression::FunctionCall(_, _) => Type::Unknown,
        }
    }

    fn parse_type_name(&self, name: &str) -> Type {
        match name {
            "int" | "i64" => Type::Integer,
            "bool" => Type::Boolean,
            "string" => Type::String,
            other => Type::Custom(other.to_string()),
        }
    }
}
