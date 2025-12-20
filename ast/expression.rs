#[derive(Debug)]
pub enum Expression {
    IntegerLiteral(i64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Variable(String),
    ArrayLiteral(Vec<(Option<String>, Expression)>), // [0 => val, "key" => val]
    ArrayAccess(Box<Expression>, Box<Expression>),   // $arr[0] or $arr["key"]
}
