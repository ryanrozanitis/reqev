use std::io::Read;
use yaml_rust2::{Yaml, YamlLoader};

pub struct FileContext {
    pub(crate) path: std::path::PathBuf,
    pub(crate) format: String,
}

pub fn read(file_ctx: FileContext) -> Vec<Yaml> {
    let file_type = &file_ctx.format.clone();
    if file_type.to_lowercase().trim() == "yaml" {
        read_yaml(file_ctx).expect("Error occurred while reading yaml file")
    } else {
        // TODO, support more file formats? json?
        panic!("Please choose a supported file format: yaml");
    }
}

fn read_yaml(file_ctx: FileContext) -> Result<Vec<Yaml>, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(file_ctx.path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to yaml read file");

    let docs = YamlLoader::load_from_str(&contents).expect("Error while parsing yaml");

    Ok(docs)
}

#[cfg(test)]
mod reader_tests {
    use super::*;
    #[test]
    fn test_correct_read_from_yaml() {
        let file_ctx = FileContext {
            format: String::from("Yaml"),
            path: std::path::PathBuf::from("tests/simple.yaml"),
        };

        let docs = read(file_ctx);
        let doc = &docs[0];

        assert_eq!("get", doc["https"][0]["type"].as_str().unwrap());
        assert_eq!("api.apis.guru", doc["https"][0]["domain"].as_str().unwrap());
        assert_eq!(
            "accept",
            doc["https"][0]["headers"][0]["key"].as_str().unwrap()
        );
        assert_eq!(
            "text/plain",
            doc["https"][0]["headers"][0]["value"].as_str().unwrap()
        );
        assert_eq!(
            200,
            doc["https"][0]["expect"][0]["status"].as_i64().unwrap()
        );
        assert_eq!("/v2/list.json", doc["https"][0]["api"][0].as_str().unwrap());
        assert_eq!(
            "/v2/list2.json",
            doc["https"][0]["api"][1].as_str().unwrap()
        );
        assert!(doc["abcd1234"].is_badvalue())
    }

    #[test]
    #[should_panic]
    fn test_bad_format_input() {
        let file_ctx = FileContext {
            format: String::from("abcd1234"),
            path: std::path::PathBuf::from("tests/simple.yaml"),
        };

        let _docs = read(file_ctx);
    }

    #[test]
    #[should_panic]
    fn test_file_not_found() {
        let file_ctx = FileContext {
            format: String::from("yaml"),
            path: std::path::PathBuf::from("abcd1234"),
        };

        let _docs = read(file_ctx);
    }
}
