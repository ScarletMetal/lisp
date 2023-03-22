use lisp::{Operator, Token};

use super::base::ParseError;
use super::context::ParseContext;

pub mod function;
pub mod flow;
pub mod values;

pub use function::*;
pub use flow::*;
pub use values::*;

pub type OperatorFactory = fn(&[Token], &mut ParseContext) -> Result<Operator, ParseError>;

