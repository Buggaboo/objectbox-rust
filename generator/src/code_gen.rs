use core::panic;
use std::collections::HashSet;

use genco::fmt;
use genco::prelude::*;

use crate::model_json::prop_type_to_impl_blanket;
use crate::model_json::ModelEntity;
use crate::model_json::ModelInfo;
use crate::model_json::ModelProperty;
use crate::ob_consts;
use crate::util::StringHelper;

fn tokens_to_string(tokens: &Tokens<Rust>) -> Vec<u8> {
    let mut w = fmt::IoWriter::new(Vec::<u8>::new());

    let fmt = fmt::Config::from_lang::<Rust>().with_indentation(fmt::Indentation::Space(4));
    let config = rust::Config::default()
        // Prettier imports and use.
        .with_default_import(rust::ImportMode::Qualified);

    if let Err(error) = tokens.format_file(&mut w.as_formatter(&fmt), &config) {
        panic!("{:?}", error);
    }

    w.into_inner()
}

trait CodeGenEntityExt {
    fn get_id_property(&self) -> Option<&ModelProperty>;
    fn generate_id_trait(&self) -> Tokens<Rust>;
    fn generate_fb_trait(&self) -> Tokens<Rust>;
    fn generate_ob_trait(&self) -> Tokens<Rust>;
    fn generate_query_trait_impls(&self) -> Tokens<Rust>;
}

fn encode_flatten(
    field_type: u32,
    flags: Option<u32>,
    offset: usize,
    name: &String,
) -> Tokens<Rust> {
    if let Some(f) = flags {
        if f == (ob_consts::OBXPropertyFlags_ID_SELF_ASSIGNABLE | ob_consts::OBXPropertyFlags_ID) {
            let t: Tokens<Rust> = quote! {
              builder.push_slot::<u64>($offset, self.$name, 0);
            };
            return t;
        }
    }

    let new_tokens: Tokens<Rust> = match field_type {
        ob_consts::OBXPropertyType_StringVector => {
            quote! {
              builder.push_slot_always($offset, vec_$offset);
            }
        }
        ob_consts::OBXPropertyType_ByteVector => {
            quote! {
              builder.push_slot_always($offset, byte_vec_$offset);
            }
        }
        ob_consts::OBXPropertyType_String => {
            quote! {
              builder.push_slot_always($offset, str_$offset);
            }
        }
        ob_consts::OBXPropertyType_Char => {
            // TODO test endianness
            quote! {
              builder.push_slot_always($offset, self.$name as u32);
            }
        }
        ob_consts::OBXPropertyType_Bool => {
            quote! {
              builder.push_slot::<bool>($offset, self.$name, false);
            }
        }
        ob_consts::OBXPropertyType_Float => {
            quote! {
              builder.push_slot::<f32>($offset, self.$name, 0.0);
            }
        }
        ob_consts::OBXPropertyType_Double => {
            quote! {
              builder.push_slot::<f64>($offset, self.$name, 0.0);
            }
        }
        _ => {
            let inferred_type_bits = match field_type {
                ob_consts::OBXPropertyType_Byte => "8",
                ob_consts::OBXPropertyType_Short => "16",
                ob_consts::OBXPropertyType_Int => "32",
                ob_consts::OBXPropertyType_Long => "64",
                _ => panic!("Unknown type"),
            };
            let is_unsigned = if let Some(f) = flags {
                if (f & ob_consts::OBXPropertyFlags_UNSIGNED)
                    == ob_consts::OBXPropertyFlags_UNSIGNED
                {
                    "u"
                } else {
                    "i"
                }
            } else {
                "i"
            };

            quote! {
              builder.push_slot::<$is_unsigned$inferred_type_bits>($offset, self.$name, 0);
            }
        }
    };
    new_tokens
}

fn encode_to_fb_unnested(field_type: u32, offset: usize, name: &String) -> Tokens<Rust> {
    let wip_offset = &rust::import("flatbuffers", "WIPOffset");

    let new_tokens: Tokens<Rust> = match field_type {
        ob_consts::OBXPropertyType_StringVector => {
            quote! {
              let strs_vec_$offset = self.$name.iter()
              .map(|s|builder.create_string(s.as_str()))
              .collect::<Vec<$wip_offset<&str>>>();
              let vec_$offset = builder.create_vector(strs_vec_$offset.as_slice());
            }
        }
        ob_consts::OBXPropertyType_ByteVector => {
            quote! {
              let byte_vec_$offset = builder.create_vector(&self.$name.as_slice());
            }
        }
        ob_consts::OBXPropertyType_String => {
            quote! {
              let str_$offset = builder.create_string(self.$name.as_str());
            }
        }
        _ => quote!(),
    };
    new_tokens
}

impl CodeGenEntityExt for ModelEntity {
    fn get_id_property(&self) -> Option<&ModelProperty> {
        for p in self.properties.iter() {
            if let Some(flags) = p.flags {
                if flags & 1 == 1 {
                    return Some(&p);
                }
            }
        }
        None
    }

    fn generate_id_trait(&self) -> Tokens<Rust> {
        let entity = &rust::import("self", &self.name);
        let obx_id = &rust::import("objectbox::c", "obx_id");
        let id_trait = &rust::import("objectbox::traits", "IdExt");

        let id = self.get_id_property();

        let p = if let Some(p) = id {
            p
        } else {
            panic!("No ID was defined for {}", self.name);
        };

        quote! {
          impl $id_trait for $entity {
            fn get_id(&self) -> $obx_id {
              self.$(p.name.as_str())
            }
            fn set_id(&mut self, id: $obx_id) {
              self.$(p.name.as_str()) = id;
            }
          }
        }
    }

    fn generate_fb_trait(&self) -> Tokens<Rust> {
        let entity = &rust::import("self", &self.name);
        let bridge_trait = &rust::import("lean_buffer::traits", "AdapterExt");
        let flatbuffer_builder = &rust::import("flatbuffers::builder", "FlatBufferBuilder");

        let unnested_props: Vec<Tokens<Rust>> = self
            .properties
            .iter()
            .enumerate()
            .map(|(i, p)| encode_to_fb_unnested(p.type_field, i * 2 + 4, &p.name))
            .collect();

        let mut props_unsorted: Vec<(usize, Tokens<Rust>)> = self
            .properties
            .iter()
            .enumerate()
            .map(|(i, p)| {
                (
                    p.to_sorting_priority(),
                    encode_flatten(p.type_field, p.flags, i * 2 + 4, &p.name),
                )
            })
            .collect();

        props_unsorted.sort_by(|a, b| a.0.cmp(&b.0));
        let props: Vec<Tokens<Rust>> = props_unsorted.iter().map(|t| t.1.clone()).collect();

        quote! {
          impl $bridge_trait for $entity {
            fn flatten(&self, builder: &mut $flatbuffer_builder) {
              builder.reset();
              $unnested_props
              let wip_offset_unfinished = builder.start_table();
              $props
              let wip_offset_finished = builder.end_table(wip_offset_unfinished);
              builder.finish_minimal(wip_offset_finished);
            }
          }
        }
    }

    fn generate_ob_trait(&self) -> Tokens<Rust> {
        // let fb_table = &rust::import("flatbuffers::table", "Table");
        let factory = &rust::import("objectbox::traits", "Factory");
        let entity_id_ext = &rust::import("objectbox::traits", "EntityIdExt");
        let factory_blanket = &rust::import("objectbox::traits", "FactoryBlanket");
        let entity = &rust::import("self", &self.name);

        let schema_id = &rust::import("objectbox::c", "obx_schema_id");

        // let destructured_props = self
        //     .properties
        //     .iter()
        //     .map(|p| p.as_struct_property_default());
        // let assigned_props = self
        //     .properties
        //     .iter()
        //     .enumerate()
        //     .map(|p| p.1.as_assigned_property(p.0 * 2 + 4));

        let mut id = String::new();
        for c in self.id.chars() {
            if c != ':' {
                id.push(c);
            } else {
                break;
            }
        }

        // TODO Store will be used for relations later
        quote! {
          impl $entity_id_ext<$entity> for $factory<$entity> {
            // fn make(&self, table: &mut $fb_table) -> $entity {
            //   let mut object = self.new_entity();
            //   // destructure
            //   let $entity {
            //     $(for p in &self.properties join (, ) => $(&p.name))
            //   } = &mut object;
            //   unsafe {
            //     $(for p in assigned_props join () => $(p))
            //   }
            //   object
            // }

            fn get_entity_id(&self) -> $schema_id {
              self.schema_id
            }

            // fn new_entity(&self) -> $entity {
            //   $entity {
            //     $(for p in destructured_props join (, ) => $(p))
            //   }
            // }
          }
          impl $factory_blanket<$(entity)> for $factory<$(entity)> {}
        }
    }

    fn generate_query_trait_impls(&self) -> Tokens<Rust> {
        let entity = &rust::import("self", &self.name);

        let cf_props = self
            .properties
            .iter()
            .map(|p| p.to_condition_factory_struct_key_value(entity));

        let cf_init_props = self
            .properties
            .iter()
            .map(|p| p.to_condition_factory_init_dyn(entity, self.id.get_id()));

        let name = self.name.as_str();
        let name_lower_case = self.name.to_ascii_lowercase();

        let vec_type_field: Vec<ob_consts::OBXPropertyType> =
            self.properties.iter().map(|p| p.type_field).collect();
        let hash_set =
            HashSet::<ob_consts::OBXPropertyType>::from_iter(vec_type_field.iter().cloned());
        let impls = hash_set
            .iter()
            .map(|t| prop_type_to_impl_blanket(*t, entity));

        quote! {
            $(for p in impls join () => $(p))
            pub struct $(name)ConditionFactory {
                $(for p in cf_props join () => $(p))
            }
            pub fn new_$(name_lower_case)_condition_factory() -> $(name)ConditionFactory {
                $(name)ConditionFactory {
                  $(for p in cf_init_props join () => $(p))
                }
            }
        }
    }
}

// TODO Fix visibility on all the trait extensions
pub(crate) trait CodeGenExt {
    fn generate_code(&self) -> String;
}

fn generate_model_fn(model_info: &ModelInfo) -> Tokens<Rust> {
    let model = &rust::import("objectbox::model", "Model");

    let tokens = &mut Tokens::<Rust>::new();

    for e in &model_info.entities {
        let entity_name = &e.name;
        let entity_id = e.id.as_comma_separated_str();
        let last_property_iduid = e.properties.last().unwrap().id.as_comma_separated_str();

        let mut props_unsorted: Vec<(usize, Tokens<Rust>)> = e
            .properties
            .iter()
            .map(|p| (p.to_sorting_priority(), p.as_fluent_builder_invocation()))
            .collect();

        props_unsorted.sort_by(|a, b| a.0.cmp(&b.0));
        let props: Vec<Tokens<Rust>> = props_unsorted.iter().map(|t| t.1.clone()).collect();

        let quote = quote! {
          .entity($(quoted(entity_name)), $entity_id)
          $props
          .last_property_id($last_property_iduid)
        };
        tokens.append(quote);
    }

    // get last_index_id
    let mut last_p_with_index_id: Option<Tokens<Rust>> = None;
    for e in model_info.entities.as_slice() {
        for p in e.properties.as_slice() {
            if let Some(x) = &p.index_id {
                last_p_with_index_id = Some(x.as_comma_separated_str());
            }
        }
    }

    let last_index_id: Tokens<Rust> = if last_p_with_index_id.is_some() {
        quote! { .last_index_id($last_p_with_index_id) }
    } else {
        quote!()
    };

    let last_entity = model_info.entities.last().unwrap();
    let last_entity_id = last_entity.id.as_comma_separated_str();

    quote! {
      pub fn make_model() -> $model {
        $model::new()
        $(tokens.clone())
        .last_entity_id($last_entity_id)
        $last_index_id
      }
    }
}

fn generate_factory_map_fn(model_info: &ModelInfo) -> Tokens<Rust> {
    let any_map = &rust::import("objectbox::map", "AnyMap");
    let factory = &rust::import("objectbox::traits", "Factory");
    let factory_helper = &rust::import("objectbox::traits", "FactoryBlanket");
    let rc = &rust::import("std::rc", "Rc");
    let phantom = &rust::import("std::marker", "PhantomData");

    let tokens = &mut Tokens::<Rust>::new();

    for e in &model_info.entities {
        let entity = &rust::import("self", &e.name);
        let mut entity_id = String::new();
        for c in e.id.chars() {
            if c != ':' {
                entity_id.push(c);
            } else {
                break;
            }
        }
        let entity_id_str = entity_id.as_str();
        let quote = quote! {
          let f$(entity_id_str) = $rc::new($factory::<$entity> {
            phantom_data: $phantom,
            schema_id: $entity_id_str
          }) as $rc<dyn $factory_helper<$entity>>;
          map.insert(f$entity_id_str);
        };
        tokens.append(quote);
    }

    quote! {
      pub fn make_factory_map() -> $any_map {
        let mut map = $any_map::new();
        $(tokens.clone())
        map
      }
    }
}

impl CodeGenExt for ModelInfo {
    fn generate_code(&self) -> String {
        let tokens = &mut rust::Tokens::new();

        for e in self.entities.iter() {
            tokens.append(e.generate_id_trait());
            // tokens.append(e.generate_fb_trait());
            tokens.append(e.generate_ob_trait());
            tokens.append(e.generate_query_trait_impls());
        }

        tokens.append(generate_model_fn(self));
        tokens.append(generate_factory_map_fn(self));

        let vector = tokens_to_string(tokens);

        let utf = std::str::from_utf8(vector.as_slice())
            .expect(
                "There is a problem with converting bytes to utf8"
            ).to_string();

        utf

        // let syntax_tree = match syn::parse_file(&utf) {
        //     Ok(parsed) => parsed,
        //     Err(error) => panic!(
        //         "There is a problem with parsing the generated rust code: {}",
        //         error
        //     ),
        // };

        // it seems that genco's code formatting is broken on stable
        // let formatted = prettyplease::unparse(&syntax_tree);

        // fs::write(&dest_path, utf)
        //     .expect(
        //         "There is a problem writing the generated rust code"
        //     );
    }
}
