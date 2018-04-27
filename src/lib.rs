// Copyright (c) 2018, Germán Fuentes Capella <german.fuentescapella@edblite.org>
// This file is licensed under the terms of BSD 2-Clause License. See the LICENSE file in the root
// of this repository for complete details.

mod result;
mod kdb;

use std::path::Path;
use result::{EDBResult, EDBError, EDBErrorType};

pub struct EDBFile {}

impl EDBFile {
    pub fn open(path: &Path) -> EDBResult<EDBFile> {
        Err(EDBError::new(EDBErrorType::NotImplemented, String::new()))
    }
}
