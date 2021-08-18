use crate::value::pod::Pod;

/// **ParsedEntity** stores the parse result.
#[derive(PartialEq, Debug)]
pub struct ParsedEntity {
    pub data: Option<Pod>,
    pub content: String,
    pub excerpt: Option<String>,
    pub orig: String,
    pub matter: String,
}

/// **ParsedEntity** stores the parse result and deserialize data to struct.
#[derive(PartialEq, Debug)]
pub struct ParsedEntityStruct<T: serde::de::DeserializeOwned> {
    pub data: T,
    pub content: String,
    pub excerpt: Option<String>,
    pub orig: String,
    pub matter: String,
}
