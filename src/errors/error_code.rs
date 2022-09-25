use std::{error::Error, fmt::Display, string::FromUtf8Error};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ErrorCode {
    E10001_IO_ERROR(std::io::Error),
    E10002_SERDE(String, serde_json::Error),
    /** mkdirs but param is file*/
    E10003_MKDIRS_NEED_DIR(String),
    E10004_WRITE_FILE(String, std::io::Error),
    E10005(FromUtf8Error),
    E10006_EXEC_PROGRAM(String, std::io::Error),
    E10007_CHMOD_FAIL(String),
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::E10001_IO_ERROR(e) => write!(f, "10001:{:?}", e),
            ErrorCode::E10002_SERDE(obj, e) => {
                write!(f, "10002: serde objet[{:?}] fail, {:?}", obj, e)
            }
            ErrorCode::E10003_MKDIRS_NEED_DIR(file) => {
                write!(f, "10003: mkdirs needs dir, but file[{:?}]", file)
            }
            ErrorCode::E10004_WRITE_FILE(file, e) => {
                write!(f, "10004: write file[{}] fail, {:?}", file, e)
            }
            ErrorCode::E10005(e) => {
                write!(f, "10005: FromUtf8Error, {:?}", e)
            }
            ErrorCode::E10006_EXEC_PROGRAM(program, e) => {
                write!(f, "10006: exec program[{}] fail, {:?}", program, e)
            }
            ErrorCode::E10007_CHMOD_FAIL(program) => {
                write!(f, "10007: chmod fail {}", program)
            }
        }
    }
}

impl Error for ErrorCode {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorCode::E10001_IO_ERROR(e) => Some(e),
            ErrorCode::E10002_SERDE(_obj, e) => Some(e),
            ErrorCode::E10003_MKDIRS_NEED_DIR(_file) => None,
            ErrorCode::E10004_WRITE_FILE(_file, e) => Some(e),
            ErrorCode::E10005(e) => Some(e),
            ErrorCode::E10006_EXEC_PROGRAM(_program, e) => Some(e),
            ErrorCode::E10007_CHMOD_FAIL(_program) => None,
        }
    }
}

impl From<std::io::Error> for ErrorCode {
    fn from(e: std::io::Error) -> Self {
        ErrorCode::E10001_IO_ERROR(e)
    }
}

impl From<FromUtf8Error> for ErrorCode {
    fn from(e: FromUtf8Error) -> Self {
        ErrorCode::E10005(e)
    }
}
