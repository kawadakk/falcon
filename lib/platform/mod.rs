use engine::engine::*;
use error::*;
use il;

pub mod linux;
pub mod linux_x86;

pub trait Platform : Clone {
    fn raise(&self, expression: &il::Expression, engine: SymbolicEngine)
    -> Result<Vec<SymbolicEngine>>;
}