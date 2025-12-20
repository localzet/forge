#[derive(Debug)]
pub struct EnumVariant {
    pub name: String,
    pub associated_data: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}
