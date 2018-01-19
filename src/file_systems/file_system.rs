
use failure::Error;

use std::io::{Cursor};
use types::BinaryData;

pub trait FileSystem:Sized {
    type RF:ReadFile;
    type WF:WriteFile;


    fn new(path:&str) -> Result<Self,Error>;
    //pub fn change_dir
    fn open_file(&mut self, file_name:&str) -> Result<Self::RF,Error>;
    fn create_file(&mut self, file_name:&str) -> Result<Self::WF,Error>;
    //fn write_file(&self, file_name:&str) -> Result<Self::WF,Error>;
}

pub trait ReadFile:Sized{
    fn read_to_end(&mut self) -> Result<BinaryData,Error>;
    fn read_to_string(&mut self) -> Result<String,Error>;
}

pub trait WriteFile:Sized{

}

