// Copyright (c) 2018, Germán Fuentes Capella <german.fuentescapella@edblite.org>
// This file is licensed under the terms of BSD 2-Clause License. See the LICENSE file in the root
// of this repository for complete details.

extern crate edblite;
extern crate url;

mod path;

use edblite::{EDBFile, EDBErrorType};
use url::Url;

#[test]
fn test_database_in_path() {
    for p in [path::for_kdb(), path::for_kdbx()].iter() {
        assert!(p.exists());
    }
}

#[test]
fn test_open_database() {
    for p in [path::for_kdb(), path::for_kdbx()].iter() {
        let result = EDBFile::open(p);
        assert_eq!(result.is_err(), true);
        assert_eq!(*result.err().unwrap().error_type(), EDBErrorType::NotImplemented);
    }
}

#[test]
fn test_not_supported_scheme() {
    let url = Url::parse("https://www.google.com").unwrap();
    let result = EDBFile::urlopen(&url);
    assert_eq!(result.is_err(), true);
    assert_eq!(*result.err().unwrap().error_type(), EDBErrorType::NotSupported);
}