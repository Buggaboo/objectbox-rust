use serde_derive::{Deserialize,Serialize};
use serde_json::Value;

use std::env;
use std::fs;
use std::path::{PathBuf, Path};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    #[serde(rename = "_note1")]
    pub note1: String,
    #[serde(rename = "_note2")]
    pub note2: String,
    #[serde(rename = "_note3")]
    pub note3: String,
    pub entities: Vec<ModelEntity>,
    pub last_entity_id: String,
    pub last_index_id: String,
    pub last_relation_id: String,
    pub last_sequence_id: String,
    pub model_version: i64,
    pub model_version_parser_minimum: i64,
    pub retired_entity_uids: Vec<Value>,
    pub retired_index_uids: Vec<Value>,
    pub retired_property_uids: Vec<Value>,
    pub retired_relation_uids: Vec<Value>,
    pub version: i64,
}

impl ModelInfo {
    pub fn from_entities(entities: &[ModelEntity]) -> Self {
      let last_entity = entities.last().unwrap(); // TODO remove unwrap, unpack result and return proper error
      let last_entity_id = last_entity.id.as_str();
      ModelInfo {
        note1: String::from("KEEP THIS FILE! Check it into a version control system (VCS) like git."),
        note2: String::from("ObjectBox manages crucial IDs for your object model. See docs for details."),
        note3: String::from("If you have VCS merge conflicts, you must resolve them according to ObjectBox docs."),
        entities: entities.to_vec(), // rehydrate from slice to vec for JSON des, all of this without cloning
        last_entity_id: last_entity_id.to_string(),
        last_index_id: String::from(""), // TODO
        last_relation_id: String::from(""), // TODO
        last_sequence_id: String::from(""), // TODO
        model_version: 5,
        model_version_parser_minimum: 5,
        retired_entity_uids: Vec::new(), // TODO
        retired_index_uids: Vec::new(), // TODO
        retired_property_uids: Vec::new(), // TODO
        retired_relation_uids: Vec::new(), // TODO
        version: 1,
      }
    }

    pub fn write(&mut self, cargo_manifest_dir: &PathBuf) {
        let dest_path = cargo_manifest_dir.as_path().join("src/objectbox-model.json");
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let result = fs::write(
                &dest_path,
                format!("{}", json),
                );
            match result {
                Err(error) => panic!("{}", error),
                _ => {}
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelEntity {
    pub id: String, // iduid = "1:12341820347123498124"
    pub last_property_id: String,
    pub name: String,
    pub properties: Vec<ModelProperty>,
    pub relations: Vec<Value>, // TODO
    // #[serde(skip_serializing_if="Option::is_none")]
    // pub path: Option<String>,
}

impl ModelEntity {
    // pub fn set_path(&mut self, path: Option<String>) -> &mut Self {
    //     self.path = path;
    //     self
    // }

    pub fn write(&mut self) {
        if let Some(out_dir) = env::var_os("OUT_DIR") {
            let dest_path = Path::new(&out_dir).join(format!("{}.objectbox.info", self.name.clone()));
            if let Ok(json) = serde_json::to_string(self) {
                let result = fs::write(
                    &dest_path,
                    format!("{}", json),
                    );
                match result {
                    Err(error) => panic!("{}", error),
                    _ => {}
                }
            }
        }else {
            panic!("Missing OUT_DIR environment variable, due to missing build.rs script");
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelProperty {
    pub id: String, // iduid = "1:12341820347123498124"
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: u16,
    #[serde(skip_serializing_if="Option::is_none")]
    pub flags: Option<u16>,
}
