
#[derive(Debug, Fail)]
pub enum DictionaryError {
    //#[fail(display = "E1: {}", name)]
    //E1 {name: String},
    #[fail(display = "{} already exists in dictionary", _0)]
    AlreadyExists(String),
    #[fail(display = "{} does not exists in dictionary", _0)]
    NoResource(String)
}