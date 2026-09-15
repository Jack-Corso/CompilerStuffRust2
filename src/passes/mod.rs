use crate::parsing::Program;

mod var_scoping;
mod var_validation;
mod validate_returns;
mod validate_yields;
mod validate_types;

pub use var_scoping::scope_vars;
pub use var_validation::validate_vars;
pub use validate_returns::validate_returns;


