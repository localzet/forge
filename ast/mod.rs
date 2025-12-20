#[derive(Debug, Clone)]
pub enum Expression {
    IntegerLiteral(i64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Variable(String),
    ArrayLiteral(Vec<(Option<String>, Box<Expression>)>), // [0 => val, "key" => val]
    ArrayAccess(Box<Expression>, Box<Expression>), // arr[index]
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: String,
        type_name: Option<String>,
        value: Expression,
    },
    Return(Option<Expression>),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<(String, Option<String>)>,
    pub return_type: Option<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

// Keep small AST; additional types (Structure, Trait, Enum) can be added later.

