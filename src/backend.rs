//! Naive Rust implementation of a flattened Hash constructed from a nested YAML document

use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

use errors::{Error, Result};

/// A `Backend` allows accessing the values of a nested YAML hash with dotted-path keys
#[derive(Debug, PartialEq)]
pub struct Backend {
    database: BTreeMap<String, String>
}

impl Backend {
    /// Create a new `Backend` from a `Path`
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Backend> {
        let mut file = try!(File::open(path));
        let mut buf = String::new();
        try!(file.read_to_string(&mut buf));

        Backend::from_str(&buf)
    }

    /// Create a new `Backend` from a `&str`
    pub fn from_str(buf: &str) -> Result<Backend> {
        let docs = try!(YamlLoader::load_from_str(&buf));
        let decoded_yaml = &docs[0];

        if let Some(yaml_hash) = decoded_yaml.as_hash() {
            let database = try!(flatten_nested_yaml_hash(yaml_hash));
            return Ok(Backend { database: database })
        }

        Err(Error::Other("Could not parse YAML document as Hash".to_string()))
    }

    /// Get the value at the dotted-path `key`
    ///
    /// Returns `Some(val)` if the `key` exists in the `database`, `None` otherwise
    pub fn get(&self, key: &String) -> Option<&String> {
        self.database.get(key)
    }

    /// Set the dotted-path `key` to the given `val`
    ///
    /// Returns `Some(old_val)` if the `key` existed in the database, `None` if it did not
    pub fn set(&mut self, key: String, val: String) -> Option<String> {
        self.database.insert(key, val)
    }
}

fn flatten_nested_yaml_hash(yaml_hash: &BTreeMap<Yaml, Yaml>) -> Result<BTreeMap<String, String>> {
    let mut database = BTreeMap::<String, String>::new();
    let _ = try!(flatten_helper(yaml_hash, &mut database, ""));
    Ok(database)
}

fn flatten_helper(yaml_hash: &BTreeMap<Yaml, Yaml>, database: &mut BTreeMap<String, String>, current_prefix: &str) -> Result<()> {
    for (key, entry) in yaml_hash.iter() {
        let skey = match key.as_str() {
            Some(key_str) => key_str,
            _ => return Err(Error::Other("Unexpected key found while parsing YAML document".to_string()))
        };

        let accumulated_key = if current_prefix != "" {
            format!("{}.{}", current_prefix, skey)
        } else {
            skey.to_string()
        };

        match *entry {
            Yaml::Hash(ref nested_hash) => { let _ = flatten_helper(nested_hash, database, &accumulated_key); },

            Yaml::String(ref translation) => { database.insert(accumulated_key, translation.to_owned()); },

            _ => { return Err(Error::Other("Unexpected value found while parsing YAML document".to_string())); }
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;

    macro_rules! treemap {
        () => {
            BTreeMap::new()
        };
        ($($k:expr => $v:expr),+) => {
            {
                let mut m = BTreeMap::new();
                $(m.insert($k, $v);)+
                m
            }
        };
    }

    #[test]
    fn test_from_str() {
        let yaml_string = r#"---
en:
  account:
    code: Code
    name: Name
  order:
    total: Price
"#;

        let database = treemap! {
            "en.account.code".to_string() => "Code".to_string(),
            "en.account.name".to_string() => "Name".to_string(),
            "en.order.total".to_string() => "Price".to_string()
        };

        let expected_backend = Backend {
            database: database
        };

        let result = Backend::from_str(yaml_string).unwrap();

        assert_eq!(expected_backend, result);
    }
}
