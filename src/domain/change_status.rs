use std::str::FromStr;

use crate::base::{Error, Result};

#[derive(PartialEq, Eq)]
pub enum ChangeStatus {
    Pending,
    InSync,
}

impl FromStr for ChangeStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<ChangeStatus> {
        match s {
            "PENDING" => Ok(ChangeStatus::Pending),
            "INSYNC" => Ok(ChangeStatus::InSync),
            _ => Err(Error::UnknownChangeStatus(s.to_owned())),
        }
    }
}
