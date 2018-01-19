
#[derive(Debug, Fail)]
pub enum StorageError {
    //#[fail(display = "E1: {}", name)]
    //E1 {name: String},
    #[fail(display = "{} does not exists", _0)]
    NoResource(String)
}