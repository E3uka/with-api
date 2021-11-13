#![warn(missing_docs)]

//! # With_api
//!
//! A simple set of macros for the ergonomic shinking of scope.
//! # Examples
//! ```
//! # use std::sync::Mutex;
//! # use std::collections::HashMap;
//! use with_api::mut_with;
//!
//! let protec: Mutex<HashMap<usize, String>> =
//!     Mutex::new(Vec::new().into_iter().collect());
//!
//! // critical section minimised.
//! mut_with!(protec.lock().unwrap(), |db| {
//!     let _ = db.insert(42, "meaning of life".to_string());
//!     assert!(!db.is_empty());
//! });
//! ```

#[macro_export]
/// Owns the value defined in the `with_expr` and evaluates the enclosing block
macro_rules! with {
    ($with_expr:expr, |$var:ident| $bl:block) => {{
        let $var = $with_expr;
        $bl()
    }};
}

#[macro_export]
/// Borrows the value defined in the `with_expr` and evaluates the enclosing block
macro_rules! bor_with {
    ($with_expr:expr, |$var:ident| $bl:block) => {{
        let $var = &$with_expr;
        $bl()
    }};
}

#[macro_export]
/// Exclusively borrows the value defined in the `with_expr` and evaluates the enclosing block
macro_rules! mut_with {
    ($with_expr:expr, |$var:ident| $bl:block) => {{
        let mut $var = $with_expr;
        $bl()
    }};
}

#[test]
fn tuple_access() {
    fn get_tup() -> (String, usize) {
        ("meaning of life".to_string(), 42)
    }

    with!(get_tup(), |tup| {
        let (description, value) = tup;
        assert!(!description.is_empty() && value == 42);
    });
}

#[test]
fn borrowed() {
    use std::collections::HashMap;
    use std::sync::Mutex;

    let db: Mutex<HashMap<&str, String>> =
        Mutex::new(vec![("key", "value".to_string())].into_iter().collect());

    bor_with!(db.lock().unwrap(), |db| {
        assert_eq!(db.get("key").unwrap(), &"value".to_string());
    });
}

#[test]
fn mutated() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    mut_with!(File::open(Path::new("test/hello.txt")).unwrap(), |file| {
        let mut buf = String::new();
        let len = file.read_to_string(&mut buf).unwrap();
        assert_ne!(len, 0);
    });
}
