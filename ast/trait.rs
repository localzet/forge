#[derive(Debug)]
pub struct Trait {
    pub name: String,
    pub methods: Vec<Function>,
}

#[derive(Debug)]
pub struct Structure {
    pub name: String,
    pub fields: Vec<(String, String)>, // (name, type)
    pub traits: Vec<String>,           // names of traits
}
