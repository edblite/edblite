// Copyright (c) 2018, Germán Fuentes Capella <german.fuentescapella@edblite.org>
// This file is licensed under the terms of BSD 2-Clause License. See the LICENSE file in the root
// of this repository for complete details.

extern crate url;

mod result;
mod kdb;

use std::path::Path;
pub use result::{EDBResult, EDBError, EDBErrorType};
use url::Url;

pub struct EDBFile {}

impl EDBFile {
    pub fn open(path: &Path) -> EDBResult<EDBFile> {
        let url = Url::from_file_path(path)?;
        EDBFile::urlopen(&url)
    }

    pub fn urlopen(url: &Url) -> EDBResult<EDBFile> {
        match url.scheme() {
            "file" => {
                println!("{}", url.path());
                Err(EDBError::new(EDBErrorType::NotImplemented, String::new()))
            },
            _ => {
                let desc = format!("No handler for scheme {}", url.scheme());
                Err(EDBError::new(EDBErrorType::NotSupported, desc))
            },
        }
    }
}
