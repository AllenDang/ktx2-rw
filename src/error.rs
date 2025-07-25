use std::fmt;

use crate::bindings::*;

/// Specialized Result type for KTX2 operations
pub type Result<T> = std::result::Result<T, Error>;

/// Comprehensive error types for KTX2 operations
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    FileDataError,
    FilePipe,
    FileOpenFailed,
    FileOverflow,
    FileReadError,
    FileSeekError,
    FileUnexpectedEof,
    FileWriteError,
    GlError,
    InvalidOperation,
    InvalidValue,
    NotFound,
    OutOfMemory,
    TranscodeFailed,
    UnknownFileFormat,
    UnsupportedTextureType,
    UnsupportedFeature,
    LibraryNotLinked,
    DecompressLengthError,
    DecompressChecksumError,
    Other(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FileDataError => {
                write!(f, "The data in the file is inconsistent with the KTX spec")
            }
            Error::FilePipe => write!(f, "The file is a pipe or named pipe"),
            Error::FileOpenFailed => write!(f, "The target file could not be opened"),
            Error::FileOverflow => write!(f, "The operation would exceed the max file size"),
            Error::FileReadError => write!(f, "An error occurred while reading from the file"),
            Error::FileSeekError => write!(f, "An error occurred while seeking in the file"),
            Error::FileUnexpectedEof => {
                write!(f, "File does not have enough data to satisfy request")
            }
            Error::FileWriteError => write!(f, "An error occurred while writing to the file"),
            Error::GlError => write!(f, "GL operations resulted in an error"),
            Error::InvalidOperation => {
                write!(f, "The operation is not allowed in the current state")
            }
            Error::InvalidValue => write!(f, "A parameter value was not valid"),
            Error::NotFound => write!(
                f,
                "Requested metadata key or required function was not found"
            ),
            Error::OutOfMemory => write!(f, "Not enough memory to complete the operation"),
            Error::TranscodeFailed => write!(f, "Transcoding of block compressed texture failed"),
            Error::UnknownFileFormat => write!(f, "The file is not a KTX file"),
            Error::UnsupportedTextureType => {
                write!(f, "The KTX file specifies an unsupported texture type")
            }
            Error::UnsupportedFeature => {
                write!(f, "Feature not included in library or not yet implemented")
            }
            Error::LibraryNotLinked => write!(f, "Library dependency not linked into application"),
            Error::DecompressLengthError => {
                write!(f, "Decompressed byte count does not match expected size")
            }
            Error::DecompressChecksumError => write!(f, "Checksum mismatch when decompressing"),
            Error::Other(code) => write!(f, "Unknown error code: {code}"),
        }
    }
}

impl std::error::Error for Error {}

#[allow(non_upper_case_globals)]
impl From<ktx_error_code_e> for Error {
    fn from(code: ktx_error_code_e) -> Self {
        match code {
            ktx_error_code_e_KTX_SUCCESS => {
                unreachable!("Success should not be converted to error")
            }
            ktx_error_code_e_KTX_FILE_DATA_ERROR => Error::FileDataError,
            ktx_error_code_e_KTX_FILE_ISPIPE => Error::FilePipe,
            ktx_error_code_e_KTX_FILE_OPEN_FAILED => Error::FileOpenFailed,
            ktx_error_code_e_KTX_FILE_OVERFLOW => Error::FileOverflow,
            ktx_error_code_e_KTX_FILE_READ_ERROR => Error::FileReadError,
            ktx_error_code_e_KTX_FILE_SEEK_ERROR => Error::FileSeekError,
            ktx_error_code_e_KTX_FILE_UNEXPECTED_EOF => Error::FileUnexpectedEof,
            ktx_error_code_e_KTX_FILE_WRITE_ERROR => Error::FileWriteError,
            ktx_error_code_e_KTX_GL_ERROR => Error::GlError,
            ktx_error_code_e_KTX_INVALID_OPERATION => Error::InvalidOperation,
            ktx_error_code_e_KTX_INVALID_VALUE => Error::InvalidValue,
            ktx_error_code_e_KTX_NOT_FOUND => Error::NotFound,
            ktx_error_code_e_KTX_OUT_OF_MEMORY => Error::OutOfMemory,
            ktx_error_code_e_KTX_TRANSCODE_FAILED => Error::TranscodeFailed,
            ktx_error_code_e_KTX_UNKNOWN_FILE_FORMAT => Error::UnknownFileFormat,
            ktx_error_code_e_KTX_UNSUPPORTED_TEXTURE_TYPE => Error::UnsupportedTextureType,
            ktx_error_code_e_KTX_UNSUPPORTED_FEATURE => Error::UnsupportedFeature,
            ktx_error_code_e_KTX_LIBRARY_NOT_LINKED => Error::LibraryNotLinked,
            ktx_error_code_e_KTX_DECOMPRESS_LENGTH_ERROR => Error::DecompressLengthError,
            ktx_error_code_e_KTX_DECOMPRESS_CHECKSUM_ERROR => Error::DecompressChecksumError,
            _ => Error::Other(code),
        }
    }
}
