


pub type InsertError<T> = std::result::Result<T, Error>; 

pub struct Error { 
    convert: ErrVariants
}

impl<T> std::convert::From<T> for Error 
where T: Into<ErrVariants>
{ 
    fn from(val: T) -> Self { 
        Self {
            convert: val.into(), 

        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrVariants { 
    #[error("Error inserting in the btree")]
    InsertionError,
}