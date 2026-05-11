use thiserror::Error;

use std::num::ParseIntError;

#[derive(Debug, Error, PartialEq)]
pub enum RutError {
    #[error("Rut doesn't follow module 11 rules.")]
    Mod11,

    #[error("Rut is too short.")]
    TooShort,

    #[error("Couldn't parse body")]
    ParseBody(#[from] ParseIntError),

    #[error("Couldn't parse dv")]
    ParseDv,
}
