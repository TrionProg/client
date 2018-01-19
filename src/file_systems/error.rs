use std;

#[derive(Debug, Fail)]
pub enum FSError {
    //#[fail(display = "E1: {}", name)]
    //E1 {name: String},
    #[fail(display = "File \"{}\" does not exists", _0)]
    NoFile(String),
    #[fail(display = "Can not read File \"{}\" does not exists", _0)]
    ReadFileError(String),
}