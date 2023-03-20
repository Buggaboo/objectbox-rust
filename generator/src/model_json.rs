use genco::prelude::rust;
use genco::prelude::Rust;
use genco::quote;
use genco::tokens::quoted;
use genco::Tokens;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use std::fs;
use std::path::PathBuf;

use crate::ob_consts;
use crate::util::StringHelper;

// TODO divide file into mod json::{info, entity, property}

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
    pub model_version: u64,
    pub model_version_parser_minimum: u64,
    pub retired_entity_uids: Vec<u64>,
    pub retired_index_uids: Vec<u64>,
    pub retired_property_uids: Vec<u64>,
    pub retired_relation_uids: Vec<u64>,
    pub version: u64,
}

impl ModelInfo {
    pub(crate) fn from_entities(slices: &[ModelEntity]) -> Self {
        let mut entities = Vec::from(slices);
        entities.sort_by(|a, b| a.name.cmp(&b.name));
        let last_entity = entities.last().unwrap(); // TODO remove unwrap, unpack result and return proper error
        let last_entity_id = last_entity.id.as_str();

        let last_property_with_index_id = entities
            .last()
            .unwrap()
            .properties
            .iter()
            .filter(|x| {
                x.index_id.is_some()
                    || (x.flags.unwrap_or_else(|| 0) & ob_consts::OBXPropertyFlags_ID) == 1
            })
            .last()
            .unwrap();
        let last_index_id = if let Some(x) = &last_property_with_index_id.index_id {
            x.to_string()
        } else {
            last_property_with_index_id.id.to_string()
        };
        ModelInfo {
        note1: String::from("KEEP THIS FILE! Check it into a version control system (VCS) like git."),
        note2: String::from("ObjectBox manages crucial IDs for your object model. See docs for details."),
        note3: String::from("If you have VCS merge conflicts, you must resolve them according to ObjectBox docs."),
        entities: entities.to_vec(), // rehydrate from slice to vec for JSON des, all of this without cloning
        last_entity_id: last_entity_id.to_string(),
        last_index_id: last_index_id.to_string(),
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

    pub(crate) fn write_json(&mut self, dest_path: &PathBuf) -> &mut Self {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            match fs::write(&dest_path, json) {
                Err(error) => panic!("Problem writing the objectbox-model.json file: {:?}", error),
                _ => {}
            }
        }
        self
    }

    pub(crate) fn from_json_file(path: &PathBuf) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(content.as_str()) {
                Ok(json) => return json,
                Err(error) => panic!("Problem parsing the json: {:?}", error),
            },
            Err(error) => panic!("Problem reading the json file: {:?}", error),
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
}

impl ModelEntity {
    pub fn write(&mut self, out_dir: &PathBuf) {
        if let Ok(json) = serde_json::to_string(self) {
            let dest_path = out_dir.join(format!("{}.objectbox.info", self.name.clone()));
            let result = fs::write(&dest_path, json.as_str());
            match result {
                Err(error) => panic!("{}", error),
                _ => {}
            }
        }
    }

    pub(crate) fn from_json_file(path: &PathBuf) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => match serde_json::from_str(content.as_str()) {
                Ok(json) => return json,
                Err(error) => panic!("Problem parsing the json: {:?}", error),
            },
            Err(error) => panic!("Problem reading the json file: {:?}", error),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelProperty {
    pub id: String, // iduid = "1:12341820347123498124"
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: ob_consts::OBXPropertyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ob_consts::OBXPropertyFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_id: Option<String>,
}

fn split_id(input: &str) -> (&str, &str) {
    let v: Vec<&str> = input.split(':').collect();
    (v[0], v[1])
}

impl ModelProperty {
    pub(crate) fn as_fluent_builder_invocation(&self) -> Tokens<Rust> {
        let flags = if let Some(f) = self.flags { f } else { 0 };
        let (id, uid) = split_id(&self.id);

        let mut q: Tokens<Rust> = quote! {
            .property(
                $(quoted(self.name.as_str())),
                $id, $uid,
                $(self.type_field),
                $flags
            )
        };
        if let Some(ii) = &self.index_id {
            let (id, uid) = split_id(&ii);
            q.extend(quote! {
                .property_index($id, $uid)
            });
        }
        q
    }

    // pub(crate) fn as_struct_property_default(&self) -> Tokens<Rust> {
    //     let name = &self.name;
    //     match self.type_field {
    //         ob_consts::OBXPropertyType_StringVector => quote! {
    //             $name: Vec::<String>::new()
    //         },
    //         ob_consts::OBXPropertyType_ByteVector => quote! {
    //             $name: Vec::<u8>::new()
    //         },
    //         ob_consts::OBXPropertyType_String => quote! {
    //             $name: String::from("")
    //         },
    //         ob_consts::OBXPropertyType_Char => quote! {
    //             $name: char::from(0)
    //         },
    //         ob_consts::OBXPropertyType_Bool => quote! {
    //             $name: false
    //         },
    //         ob_consts::OBXPropertyType_Float => quote! {
    //             $name: 0.0
    //         },
    //         ob_consts::OBXPropertyType_Double => quote! {
    //             $name: 0.0
    //         },
    //         // rest of the integer types
    //         _ => quote! {
    //             $name: 0
    //         },
    //     }
    // }

    // pub(crate) fn as_assigned_property(&self, offset: usize) -> Tokens<Rust> {
    //     let fuo = &rust::import("flatbuffers", "ForwardsUOffset");
    //     let fvec = &rust::import("flatbuffers", "Vector");

    //     let name = &self.name;
    //     if let Some(f) = self.flags {
    //         if f == (ob_consts::OBXPropertyFlags_ID_SELF_ASSIGNABLE
    //             | ob_consts::OBXPropertyFlags_ID)
    //         {
    //             let t: Tokens<Rust> = quote! {
    //                 *$name = table.get::<u64>($offset, Some(0)).unwrap();
    //             };
    //             return t;
    //         }
    //     }

    //     let name = &self.name;
    //     match self.type_field {
    //         ob_consts::OBXPropertyType_StringVector => quote! {
    //             let fb_vec_$name = table.get::<$fuo<$fvec<$fuo<&str>>>>($offset, None);
    //             if let Some(sv) = fb_vec_$name {
    //                 *$name = sv.iter().map(|s|s.to_string()).collect();
    //             }
    //         },
    //         ob_consts::OBXPropertyType_ByteVector => quote! {
    //             let fb_vec_$name = table.get::<$fuo<$fvec<u8>>>($offset, None);
    //             if let Some(bv) = fb_vec_$name {
    //                 *$name = bv.bytes().to_vec();
    //             }
    //         },
    //         // TODO research clear the buffer, and read the slice instead
    //         // TODO see what's faster
    //         ob_consts::OBXPropertyType_String => quote! {
    //             if let Some(s) = table.get::<$fuo<&str>>($offset, None) {
    //                 *$name = s.to_string();
    //             }
    //         },
    //         // TODO will this work with objectbox? rust char = 4x u8 = 32 bits
    //         // TODO write test for this specifically
    //         ob_consts::OBXPropertyType_Char => quote! {
    //             let $(name)_u32 = table.get::<u32>($offset, Some(0)).unwrap();
    //             if let Some(c) = std::char::from_u32($(name)_u32) {
    //                 *$name = c;
    //             }
    //         },
    //         ob_consts::OBXPropertyType_Bool => quote! {
    //             *$name = table.get::<bool>($offset, Some(false)).unwrap();
    //         },
    //         ob_consts::OBXPropertyType_Float => quote! {
    //             *$name = table.get::<f32>($offset, Some(0.0)).unwrap();
    //         },
    //         ob_consts::OBXPropertyType_Double => quote! {
    //             *$name = table.get::<f64>($offset, Some(0.0)).unwrap();
    //         },
    //         // rest of the integer types
    //         _ => {
    //             let unsigned_flag = match self.flags {
    //                 Some(f) => f,
    //                 _ => 0,
    //             };
    //             let sign: Tokens<Rust> = if (unsigned_flag & ob_consts::OBXPropertyFlags_UNSIGNED)
    //                 == ob_consts::OBXPropertyFlags_UNSIGNED
    //             {
    //                 quote!(u)
    //             } else {
    //                 quote!(i)
    //             };

    //             let bits: Tokens<Rust> = match self.type_field {
    //                 ob_consts::OBXPropertyType_Byte => quote!(8),
    //                 ob_consts::OBXPropertyType_Short => quote!(16),
    //                 ob_consts::OBXPropertyType_Int => quote!(32),
    //                 ob_consts::OBXPropertyType_Long => quote!(64),
    //                 _ => panic!("Unknown OBXPropertyType"),
    //             };
    //             quote! {
    //                 *$name = table.get::<$sign$bits>($offset, Some(0)).unwrap();
    //             }
    //         }
    //     }
    // }

    pub(crate) fn to_sorting_priority(&self) -> usize {
        match self.type_field {
            ob_consts::OBXPropertyType_Double => 1,
            ob_consts::OBXPropertyType_Long => 1,
            ob_consts::OBXPropertyType_StringVector => 2,
            ob_consts::OBXPropertyType_ByteVector => 3,
            ob_consts::OBXPropertyType_String => 4,
            ob_consts::OBXPropertyType_Float => 5,
            ob_consts::OBXPropertyType_Int => 5,
            ob_consts::OBXPropertyType_Char => 5,
            ob_consts::OBXPropertyType_Short => 6,
            ob_consts::OBXPropertyType_Bool => 7,
            ob_consts::OBXPropertyType_Byte => 7,
            _ => 8, // TODO refine this for the remaining types, no support for now
        }
    }

    pub(crate) fn to_condition_factory_struct_key_value(
        &self,
        entity_name: &genco::lang::rust::Import,
    ) -> Tokens<Rust> {
        let type_double =
            &rust::import("objectbox::query::traits", "F64Blanket").with_module_alias("qtraits");
        let type_float =
            &rust::import("objectbox::query::traits", "F32Blanket").with_module_alias("qtraits");
        let type_long =
            &rust::import("objectbox::query::traits", "I64Blanket").with_module_alias("qtraits");
        let type_int =
            &rust::import("objectbox::query::traits", "I32Blanket").with_module_alias("qtraits");
        let type_char =
            &rust::import("objectbox::query::traits", "CharBlanket").with_module_alias("qtraits");
        let type_short =
            &rust::import("objectbox::query::traits", "I16Blanket").with_module_alias("qtraits");
        let type_bool =
            &rust::import("objectbox::query::traits", "BoolBlanket").with_module_alias("qtraits");
        let type_byte =
            &rust::import("objectbox::query::traits", "I8Blanket").with_module_alias("qtraits");
        let type_byte_vec =
            &rust::import("objectbox::query::traits", "VecU8Blanket").with_module_alias("qtraits");
        let type_string =
            &rust::import("objectbox::query::traits", "StringBlanket").with_module_alias("qtraits");
        let name = &self.name;
        match self.type_field {
            ob_consts::OBXPropertyType_Double => quote! {
                pub $name: Box<dyn $type_double<$entity_name>>,
            },
            ob_consts::OBXPropertyType_Long => quote! {
                pub $name: Box<dyn $type_long<$entity_name>>,
            },
            ob_consts::OBXPropertyType_ByteVector => quote! {
                pub $name: Box<dyn $type_byte_vec<$entity_name>>,
            },
            ob_consts::OBXPropertyType_String => quote! {
                pub $name: Box<dyn $type_string<$entity_name>>,
            },
            ob_consts::OBXPropertyType_Float => quote! {
                pub $name: Box<dyn $type_float<$entity_name>>,
            },
            ob_consts::OBXPropertyType_Int => quote! {
                pub $name: Box<dyn $type_int<$entity_name>>,
            },
            ob_consts::OBXPropertyType_Char => quote! {
                pub $name: Box<dyn $type_char<$entity_name>>,
            },
            ob_consts::OBXPropertyType_Short => quote! {
                pub $name: Box<dyn $type_short<$entity_name>>,
            },
            ob_consts::OBXPropertyType_Bool => quote! {
                pub $name: Box<dyn $type_bool<$entity_name>>,
            },
            ob_consts::OBXPropertyType_Byte => quote! {
                pub $name: Box<dyn $type_byte<$entity_name>>,
            },
            _ => quote!(), // TODO refine this for the remaining types, no support for now
        }
    }

    pub(crate) fn to_condition_factory_init_dyn(
        &self,
        entity_name: &genco::lang::rust::Import,
        entity_id: Tokens<Rust>,
    ) -> Tokens<Rust> {
        let ccb_fn = &rust::import("objectbox::query::traits", "create_condition_builder")
            .with_module_alias("qtraits");

        let name = &self.name;
        let property_id = &self.id.get_id();

        match self.type_field {
            ob_consts::OBXPropertyType_Double
            | ob_consts::OBXPropertyType_Long
            | ob_consts::OBXPropertyType_ByteVector
            | ob_consts::OBXPropertyType_String
            | ob_consts::OBXPropertyType_Float
            | ob_consts::OBXPropertyType_Int
            | ob_consts::OBXPropertyType_Char
            | ob_consts::OBXPropertyType_Short
            | ob_consts::OBXPropertyType_Bool
            | ob_consts::OBXPropertyType_Byte => quote! {
                $name: Box::new($ccb_fn::<$entity_name, $entity_id, $(property_id), $(self.type_field)>()),
            },
            _ => quote!(), // TODO refine this for the remaining types, no support for now
        }
    }
}

/// Use unique set of OBXPropertyType to generate the required blankets
pub(crate) fn prop_type_to_impl_blanket(
    type_field: ob_consts::OBXPropertyType,
    entity_name: &genco::lang::rust::Import,
) -> Tokens<Rust> {
    let impl_double =
        &rust::import("objectbox::query::traits", "F64Blanket").with_module_alias("qtraits");
    let impl_float =
        &rust::import("objectbox::query::traits", "F32Blanket").with_module_alias("qtraits");
    let impl_long =
        &rust::import("objectbox::query::traits", "I64Blanket").with_module_alias("qtraits");
    let impl_int =
        &rust::import("objectbox::query::traits", "I32Blanket").with_module_alias("qtraits");
    let impl_char =
        &rust::import("objectbox::query::traits", "CharBlanket").with_module_alias("qtraits");
    let impl_short =
        &rust::import("objectbox::query::traits", "I16Blanket").with_module_alias("qtraits");
    let impl_bool =
        &rust::import("objectbox::query::traits", "BoolBlanket").with_module_alias("qtraits");
    let impl_byte =
        &rust::import("objectbox::query::traits", "I8Blanket").with_module_alias("qtraits");
    let impl_byte_vec =
        &rust::import("objectbox::query::traits", "VecU8Blanket").with_module_alias("qtraits");
    let impl_string =
        &rust::import("objectbox::query::traits", "StringBlanket").with_module_alias("qtraits");

    let cb =
        &rust::import("objectbox::query::traits", "ConditionBuilder").with_module_alias("qtraits");
    match type_field {
        ob_consts::OBXPropertyType_Double => {
            quote! {
                impl $impl_double<$entity_name> for $cb<$entity_name> {}
            }
        }
        ob_consts::OBXPropertyType_Long => {
            quote! {
                impl $impl_long<$entity_name> for $cb<$entity_name> {}
            }
        }
        ob_consts::OBXPropertyType_ByteVector => {
            quote! {
                impl $impl_byte_vec<$entity_name> for $cb<$entity_name> {}
            }
        }
        ob_consts::OBXPropertyType_String => {
            quote! {
                impl $impl_string<$entity_name> for $cb<$entity_name> {}
            }
        }
        ob_consts::OBXPropertyType_Float => {
            quote! {
                impl $impl_float<$entity_name> for $cb<$entity_name> {}
            }
        }
        ob_consts::OBXPropertyType_Int => {
            quote! {
                impl $impl_int<$entity_name> for $cb<$entity_name> {}
            }
        }
        ob_consts::OBXPropertyType_Char => {
            quote! {
                impl $impl_char<$entity_name> for $cb<$entity_name> {}
            }
        }
        ob_consts::OBXPropertyType_Short => {
            quote! {
                impl $impl_short<$entity_name> for $cb<$entity_name> {}
            }
        }
        ob_consts::OBXPropertyType_Bool => {
            quote! {
                impl $impl_bool<$entity_name> for $cb<$entity_name> {}
            }
        }
        ob_consts::OBXPropertyType_Byte => {
            quote! {
                impl $impl_byte<$entity_name> for $cb<$entity_name> {}
            }
        }
        // ob_consts::OBXPropertyType_StringVector => 2,
        _ => quote!(), // TODO refine this for the remaining types, no support for now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_mp() -> ModelProperty {
        ModelProperty {
            id: "1:2".to_string(),
            name: "name".to_string(),
            type_field: 3,
            flags: Some(0),
            index_id: Some("2:3".to_string()),
        }
    }

    #[test]
    fn model_property_fluent_builder_test() {
        let mp = new_mp();
        let str = mp
            .as_fluent_builder_invocation()
            .to_string()
            .expect("valid");
        assert_eq!(
            str,
            ".property( \"name\", 1, 2, 3, 0 ).property_index(2, 3)"
        );
    }

    #[test]
    fn condition_builder_struct_test() {
        let mp = new_mp();
        let entity_name = &rust::import("crate", "some_entity");
        let struct_a = quote! {
            struct A<'a> {
                $(mp.to_condition_factory_struct_key_value(entity_name))
            }
        };
        assert_eq!(
            "struct A<'a> { name: &'a dyn I16Blanket<some_entity> }",
            struct_a.to_string().expect("meh")
        );

        // let code = structA.to_string().expect("explode");
        // let code_parsed = syn::parse_str::<DeriveInput>(code.as_str()).expect("crash");
    }

    #[test]
    fn condition_builder_init_struct_test() {
        let mp = new_mp();
        let entity_name = &rust::import("crate", "some_entity");
        let id = mp.id.get_id();
        let struct_a = quote! {
            A {
                $(mp.to_condition_factory_init_dyn_cast(entity_name, id))
            }
        };
        assert_eq!("A { name: &create_condition_builder::<some_entity, 1, 1, 3> as &dyn I16Blanket<some_entity> }", struct_a.to_string().expect("meh"));
    }
}
