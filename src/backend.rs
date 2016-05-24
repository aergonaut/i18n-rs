use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

use errors::{Error, Result};

#[derive(Debug, PartialEq)]
pub struct Backend {
    database: BTreeMap<String, String>
}

impl Backend {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Backend> {
        let mut file = try!(File::open(path));
        let mut buf = String::new();
        try!(file.read_to_string(&mut buf));

        Backend::from_str(&buf)
    }

    pub fn from_str(buf: &str) -> Result<Backend> {
        let decoded_yaml = &YamlLoader::load_from_str(&buf)?[0];

        if let Some(yaml_hash) = decoded_yaml.as_hash() {
            let database = try!(flatten_nested_yaml_hash(yaml_hash));
            return Ok(Backend { database: database })
        }

        Err(Error::Other("Could not parse YAML document as Hash".to_string()))
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
