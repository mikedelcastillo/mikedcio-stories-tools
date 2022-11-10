use std::{env, path::PathBuf, io::{BufReader, Read, Cursor}, fs::File};

use anyhow::{Result, Error};
use suppaftp::{FtpStream, types::FileType};

use crate::filter_ignored;

pub trait Remote {
    fn list() -> Result<Vec<String>>;
    fn upload(file_path: PathBuf) -> Result<()>;
}

pub struct FTPRemote;

impl FTPRemote {
    pub fn get_connection() -> Result<FtpStream> {
        let server = env::var("FTP_SERVER")?;
        let username = env::var("FTP_USERNAME")?;
        let password = env::var("FTP_PASSWORD")?;
        
        let mut ftp_stream = FtpStream::connect(server)?;
        let _ = ftp_stream.login(username.as_str(), password.as_str())?;

        Ok(ftp_stream)
    }
}

impl Remote for FTPRemote {
    fn list() -> Result<Vec<String>> {
        let mut ftp = Self::get_connection()?;
        let list = ftp.nlst(None)?;
        let list = filter_ignored(list);
        
        let _ = ftp.quit()?;
        Ok(list)
    }

    fn upload(file_path: PathBuf) -> Result<()> {
        let mut ftp = Self::get_connection()?;

        let file = std::fs::read(&file_path)?;
        let mut reader = Cursor::new(file);

        let file_name = file_path.file_name()
            .ok_or(Error::msg("Cannot get file_name"))?
            .to_str()
            .ok_or(Error::msg("Could not convert file_name OsStr to str"))?;

        let _ = ftp.transfer_type(FileType::Binary)?;
        let _ = ftp.put_file(file_name, &mut reader)?;

        let _ = ftp.quit()?;
        Ok(())
    }
}

pub type ActiveRemote = FTPRemote;