// Copyright 2016 Serde YAML Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde;
extern crate serde_yaml;

use std::fmt::Debug;
use std::collections::BTreeMap;

fn test_de<T>(yaml: &str, expected: T)
    where T: serde::Deserialize + PartialEq + Debug,
{
    let deserialized: T = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(expected, deserialized);
}

#[test]
fn test_alias() {
    let yaml = indoc!("
        ---
        first:
          &alias
          1
        second:
          *alias");
    let mut expected = BTreeMap::new();
    {
        expected.insert(String::from("first"), 1);
        expected.insert(String::from("second"), 1);
    }
    test_de(yaml, expected);
}

#[test]
fn test_option() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Data {
        a: Option<f64>,
        b: Option<String>,
        c: Option<bool>,
    }
    let yaml = indoc!("
        ---
        b:
        c: true");
    let expected = Data {
        a: None,
        b: None,
        c: Some(true),
    };
    test_de(yaml, expected);
}
