// Copyright (c) 2018, Germán Fuentes Capella <german.fuentescapella@edblite.org>
// This file is licensed under the terms of BSD 2-Clause License. See the LICENSE file in the root
// of this repository for complete details.

use std::path::PathBuf;

fn _for(rel_path: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(String::from(rel_path));
    path
}

const PATH_KDB: &str = "tests/databases/Test_Password_1234.kdb";
const PATH_KDBX: &str = "tests/databases/Test_Password_1234.kdbx";

pub fn for_kdb() -> PathBuf {
    _for(PATH_KDB)
}

pub fn for_kdbx() -> PathBuf {
    _for(PATH_KDBX)
}