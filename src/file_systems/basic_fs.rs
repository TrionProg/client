use std;

use std::io::{Read,Write,Seek};
use std::io::Result as IOResult;

use types::BinaryData;

use failure::Error;

use super::{FileSystem,ReadFileSystem,WriteFileSystem};
use super::{File,ReadFile,WriteFile};

#[derive(Debug, Fail)]
pub enum BasicFSError {
    #[fail(display = "Can not open file \"{}\" : {}", _0, _1)]
    CanNotOpenFile(String, std::io::Error),
    #[fail(display = "Can not create file \"{}\" : {}", _0, _1)]
    CanNotCreateFile(String, std::io::Error),
    #[fail(display = "No metadata for file \"{}\" : {}", _0, _1)]
    NoMetadata(String, std::io::Error),
    #[fail(display = "Type of file \"{}\" is not 'file'", _0)]
    IsNotFile(String),
    #[fail(display = "File \"{}\" is directory", _0)]
    IsDirectory(String),
    #[fail(display = "Can not read file \"{}\" : {}", _0, _1)]
    CanNotReadFile(String, std::io::Error),
    #[fail(display = "Not all bytes of file \"{}\" were read", _0)]
    NotAllBytesRead(String),
    #[fail(display = "Can not write file \"{}\" : {}", _0, _1)]
    CanNotWriteFile(String, std::io::Error),
}

pub struct BasicFS {
    path:String,
}

impl BasicFS {
    pub fn new(path:&str) -> Result<Self,Error> {
        //TODO: check path

        let fs=BasicFS {
            path:path.to_string()
        };

        ok!(fs)
    }
}

impl FileSystem for BasicFS {
    fn get_path(&self) -> &str {
        self.path.as_str()
    }
}

impl<'a> ReadFileSystem<'a> for BasicFS {
    type RF=BasicFile;

    fn open_file(&mut self, file_name:&str) -> Result<Self::RF,Error> {
        let path=format!("{}/{}", self.path, file_name);
        let file=match std::fs::File::open(path.as_str()) {
            Ok(file) => file,
            Err(e) => bail!(BasicFSError::CanNotOpenFile(path,e))
        };

        BasicFile::new(file,path)
    }
}

impl WriteFileSystem for BasicFS {
    type WF=BasicFile;

    fn create_file(&mut self, file_name:&str) -> Result<Self::WF,Error> {
        let path=format!("{}/{}", self.path, file_name);
        let file=match std::fs::File::create(path.as_str()) {
            Ok(file) => file,
            Err(e) => bail!(BasicFSError::CanNotCreateFile(path,e))
        };

        BasicFile::new(file,path)
    }
}

pub struct BasicFile {
    file:std::fs::File,
    path:String,
    len:u64,
}

impl BasicFile {
    fn new(file:std::fs::File, path:String) -> Result<Self,Error>{
        let metadata=match file.metadata() {
            Ok(metadata) => metadata,
            Err(e) => bail!(BasicFSError::NoMetadata(path,e))
        };

        if !metadata.is_file() {
            if metadata.is_dir() {
                bail!(BasicFSError::IsDirectory(path))
            }

            bail!(BasicFSError::IsNotFile(path))
        }

        let len=metadata.len();

        let file=BasicFile{
            file,
            path,
            len
        };

        ok!(file)
    }
}

impl Read for BasicFile{
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> { self.file.read(buf) }
}

impl Write for BasicFile{
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> { self.file.write(buf) }
    fn flush(&mut self) -> IOResult<()> { self.file.flush() }
}

impl Seek for BasicFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> IOResult<u64> { self.file.seek(pos) }
}

impl File for BasicFile {
    fn get_path(&self) -> &str {
        self.path.as_str()
    }
}

impl ReadFile for BasicFile {
    fn read_to_end(&mut self) -> Result<BinaryData,Error> {
        let mut buf=Vec::with_capacity(self.len as usize);

        let bytes_read=self.file.read_to_end(&mut buf).map_err(|e|BasicFSError::CanNotReadFile(self.path.clone(),e))?;

        if bytes_read!=self.len as usize {
            bail!(BasicFSError::NotAllBytesRead(self.path.clone()))
        }

        ok!(buf)
    }

    fn read_to_string(&mut self) -> Result<String,Error> {
        let mut buf=String::with_capacity(self.len as usize);

        let bytes_read=self.file.read_to_string(&mut buf).map_err(|e|BasicFSError::CanNotReadFile(self.path.clone(),e))?;

        if bytes_read!=self.len as usize {
            bail!(BasicFSError::NotAllBytesRead(self.path.clone()))
        }

        ok!(buf)
    }
}

impl WriteFile for BasicFile {

}