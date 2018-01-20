
use std;
use std::marker::PhantomData;
use std::io::{Read,Write,Seek};

use types::BinaryData;

use zip;

use failure::Error;

use super::{FileSystem, ReadFileSystem};
use super::{File, ReadFile};

#[derive(Debug, Fail)]
pub enum ZipError {
    #[fail(display = "Can not open archive \"{}\" : {}", _0, _1)]
    CanNotOpenArchive(String, zip::result::ZipError),
    #[fail(display = "Can not open file in archive \"{}\" : {}", _0, _1)]
    CanNotOpenFile(String, zip::result::ZipError),
    #[fail(display = "Can not read file \"{}\" : {}", _0, _1)]
    CanNotReadFile(String, std::io::Error),
    #[fail(display = "Not all bytes of file \"{}\" were read", _0)]
    NotAllBytesRead(String),
}

pub struct ReadZipArchive<'a, FS:ReadFileSystem<'a>> where FS::RF:Read + Seek{
    path:String,
    archive:zip::read::ZipArchive<FS::RF>,
    _phantom_data:PhantomData<FS>
}

impl<'a, FS:ReadFileSystem<'a>> ReadZipArchive<'a, FS> where FS::RF:Read + Seek{
    pub fn open(file_system:&mut FS, file_name:&str) -> Result<Self,Error> {
        let file=file_system.open_file(file_name)?;
        let path=file.get_path().to_string();

        let archive=match zip::read::ZipArchive::new(file) {
            Ok(archive) => archive,
            Err(e) => bail!(ZipError::CanNotOpenArchive(path,e))
        };

        let archive=ReadZipArchive {
            path,
            archive,
            _phantom_data:PhantomData
        };

        ok!(archive)
    }
}

impl<'a, FS:ReadFileSystem<'a>> FileSystem for ReadZipArchive<'a, FS> where FS::RF:Read + Seek{
    fn get_path(&self) -> &str {
        self.path.as_str()
    }
}

impl<'a,FS:ReadFileSystem<'a>> ReadFileSystem<'a> for ReadZipArchive<'a, FS> where FS::RF:Read + Seek{
    type RF = ReadZipFile<'a>;

    fn open_file(&mut self, file_name:&str) -> Result<Self::RF,Error> {
        let path=format!("{}/{}", self.path, file_name);
        let file=match self.archive.by_name(file_name) {
            Ok(file) => file,
            Err(e) => bail!(ZipError::CanNotOpenFile(path,e))
        };

        ReadZipFile::new(file,path)
    }
}

pub struct ReadZipFile<'a>{
    file:zip::read::ZipFile<'a>,
    path:String,
    len:u64,
}

impl<'a> ReadZipFile<'a> {
    fn new(file:zip::read::ZipFile<'a>, path:String) -> Result<Self,Error> {
        let len=file.size();

        let file=ReadZipFile {
            file,
            path,
            len
        };

        ok!(file)
    }
}

impl<'a> File for ReadZipFile<'a> {
    fn get_path(&self) -> &str {
        self.path.as_str()
    }
}

impl<'a> ReadFile for ReadZipFile<'a> {
    fn read_to_end(&mut self) -> Result<BinaryData,Error> {
        let mut buf=Vec::with_capacity(self.len as usize);

        let bytes_read=self.file.read_to_end(&mut buf).map_err(|e|ZipError::CanNotReadFile(self.path.clone(),e))?;

        if bytes_read!=self.len as usize {
            bail!(ZipError::NotAllBytesRead(self.path.clone()))
        }

        ok!(buf)
    }

    fn read_to_string(&mut self) -> Result<String,Error> {
        let mut buf=String::with_capacity(self.len as usize);

        let bytes_read=self.file.read_to_string(&mut buf).map_err(|e|ZipError::CanNotReadFile(self.path.clone(),e))?;

        if bytes_read!=self.len as usize {
            bail!(ZipError::NotAllBytesRead(self.path.clone()))
        }

        ok!(buf)
    }
}