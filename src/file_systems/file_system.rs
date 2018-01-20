
use failure::Error;

use std::io::{Cursor};
use types::BinaryData;

pub trait FileSystem:Sized {
    fn get_path(&self) -> &str;
}

pub trait ReadFileSystem<'a>:FileSystem {
    type RF:ReadFile+'a;


    //fn new(path:&str) -> Result<Self,Error>;
    //pub fn change_dir
    fn open_file(&'a mut self, file_name:&str) -> Result<Self::RF,Error>;
    //fn write_file(&self, file_name:&str) -> Result<Self::WF,Error>;
}

pub trait WriteFileSystem<'a>:FileSystem {
    type WF:WriteFile+'a;


    //fn new(path:&str) -> Result<Self,Error>;
    //pub fn change_dir
    fn create_file(&'a mut self, file_name:&str) -> Result<Self::WF,Error>;
    //fn write_file(&self, file_name:&str) -> Result<Self::WF,Error>;
}

pub trait File:Sized {
    fn get_path(&self) -> &str;
}

pub trait ReadFile:File{
    fn read_to_end(&mut self) -> Result<BinaryData,Error>;
    fn read_to_string(&mut self) -> Result<String,Error>;
}

pub trait WriteFile:File{

}

