//! Contains the 'translations' into C code
//! 
//! _Yup, this module also need a better name_

pub trait CSyntax {
    fn to_c_syntax(&self) -> String;
}

mod operators;
mod literals;