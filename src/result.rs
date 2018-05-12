// Copyright (c) 2018, Germán Fuentes Capella <german.fuentescapella@edblite.org>
// This file is licensed under the terms of BSD 2-Clause License. See the LICENSE file in the root
// of this repository for complete details.

#[derive(Debug, PartialEq)]
pub enum EDBErrorType {
    NotImplemented,
    NotSupported,
}

#[derive(Debug, PartialEq)]
pub struct EDBError {
    error_type: EDBErrorType,
    description: String,
}

pub type EDBResult<T> = Result<T, EDBError>;

impl EDBError {
    pub fn new(error_type: EDBErrorType, description: String) -> Self {
        EDBError { error_type, description }
    }

    pub fn error_type(&self) -> &EDBErrorType {
        &self.error_type
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

impl From<()> for EDBError {
    // This error should not happen
    fn from(_error: ()) -> Self {
        panic!("Ups, this is embarrassing");
    }
}