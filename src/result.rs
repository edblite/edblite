// Copyright (c) 2018, Germán Fuentes Capella <german.fuentescapella@edblite.org>
// This file is licensed under the terms of BSD 2-Clause License. See the LICENSE file in the root
// of this repository for complete details.

pub enum EDBErrorType {
    NotImplemented,
}

pub struct EDBError {
    error_type: EDBErrorType,
    description: String,
}

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

pub type EDBResult<T> = Result<T, EDBError>;