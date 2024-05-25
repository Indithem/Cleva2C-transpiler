//! _Please suggest some better name for this module..!_
//! 
//! This module shall contain the structs and functions neccessary to 
//! transpile Cleva to C.
//! i.e., print the ast into a valid C code.

use super::ast;

mod transpiler;
mod c_translations;

#[cfg(test)]
mod unit_tests;

pub use transpiler::Transpiler;