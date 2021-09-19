use std::fmt;

use druid::Data;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    RequestFail(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::RequestFail(err) => f.write_str(err),
        }
    }
}

impl Data for Error {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}
