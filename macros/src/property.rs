use std::option::Option;

use objectbox_generator::id;
use objectbox_generator::ob_consts as consts;

use crate::IdUidMacroHelper;
use crate::path_visitor::get_idents_from_path;

// TODO implement flags, reference: https://github.com/objectbox/objectbox-dart/blob/main/generator/lib/src/entity_resolver.dart#L23-L30

#[derive(Debug)]
pub struct Property {
  pub name: String,
  pub field_type: consts::OBXPropertyType,
  pub id: id::IdUid,
  pub flags: consts::OBXPropertyFlags,
  pub index_id: Option<String>,
}

impl Property {

  pub(crate) fn new() -> Self {
    Property {
        name: String::new(),
        field_type: 0,
        id: id::IdUid::zero(),
        flags: 0,
        index_id: None,
    }
  }

  pub(crate) fn scan_obx_property_type_and_flags (mnv: &syn::MetaNameValue) -> (consts::OBXPropertyType, consts::OBXPropertyFlags) {
    let mut obx_property_type: consts::OBXPropertyType = 0;
    let mut obx_property_flags: consts::OBXPropertyFlags = 0;

    if let syn::Lit::Int(li) = &mnv.lit {
      let result = li.base10_parse::<consts::OBXPropertyFlags>();
      if let Ok(value) = result {
        if let Some(ident) = mnv.path.get_ident() {
          let param_name: &str = &ident.to_string();
          match param_name {
            "type" => { obx_property_type = value },
            "flags" => { obx_property_flags = value }
            _ => {}
          }
        }
      }
    }
    (obx_property_type, obx_property_flags)
  }


  pub(crate) fn from_syn_field(field: &syn::Field) -> Option<Property> {
    let mut property = Property::new();

    let Property {
      name,
      field_type: obx_property_type,
      id,
      flags: obx_property_flags,
      index_id
    } = &mut property;
    
    if let Some(ident) = &field.ident {
      let new_name = ident.to_string();
      name.push_str(&new_name);

      // print_field_token_stream(field, new_name);

      // TODO Document: for the minimal demo, ensure entities are pub
      // TODO scan: i.e. parse them for pub keyword
      // TODO declared on the src/lib.rs or src/main.rs and are pub
      // Attribute parsing
      // TODO more error checking, certain combinations shouldn't be allowed
      for a in field.attrs.iter() {
        // get attribute name from `#[name]`
        if let Some(attr_path_ident) = a.path.get_ident() {
          let attr_name : &str = &attr_path_ident.to_string();
          // TODO add safety precaution measures
          // TODO add extra parameters
          match attr_name {
            "id" => {
              *obx_property_type = consts::OBXPropertyType_Long;
              *obx_property_flags |= consts::OBXPropertyFlags_ID_SELF_ASSIGNABLE | consts::OBXPropertyFlags_ID;
              return Some(property);
            }
            "index" => {
              *obx_property_flags |= consts::OBXPropertyFlags_INDEXED | consts::OBXPropertyFlags_UNIQUE; // 40
              *index_id = Some("0:0".to_owned());
            }, // id, uid, type
            "unique" => {
              *obx_property_flags |= consts::OBXPropertyFlags_UNIQUE;
              *index_id = Some("0:0".to_owned());
            }, // id, uid, type
            "backlink" => {},
            // "transient" => { quote::__private::ext::RepToTokensExt::next(&a); }, // TODO test if this really skips
            "property" => {}, // id, uid, type, flags
            _ => {
              // skip if not ours
              continue;
            }
          }  
        }

        // TODO move out as generalized function with lambda
        // that parses depending on given attrib parameter names
        // given by 'index', 'backlink', 'transient', 'property'
        if let syn::parse::Result::Ok(m) = a.parse_meta() {
          match m {
            // single parameter
            syn::Meta::NameValue(mnv) => {
              id.update_from_scan(&mnv);
              (*obx_property_type, *obx_property_flags) = Self::scan_obx_property_type_and_flags(&mnv);
            },
            // multiple parameters
            syn::Meta::List(meta_list) => {
              meta_list.nested.into_iter().for_each(|nm| {
                if let syn::NestedMeta::Meta(meta) = nm {
                  if let syn::Meta::NameValue(mnv) = meta {
                    id.update_from_scan(&mnv);
                    (*obx_property_type, *obx_property_flags) = Self::scan_obx_property_type_and_flags(&mnv);
                  }
                }
              });
            },
            /* syn::Meta::Path(path) */ _ => {}
          }
        }
      }

      let idents = get_idents_from_path(&field.ty);
      let ident_joined = idents.iter().map(|i|i.to_string()).collect::<String>();
      let ident = ident_joined.as_str();

      // TODO discuss support for Option<Primitive>,
      // index is a special case, where index == 0, has a special meaning
      // wrt to the Store and Box (not rust's)
      // e.g. the indent.as_str() would be for Option<String> => OptionString

      *obx_property_type = match ident {
        "bool" => consts::OBXPropertyType_Bool,
        "i8" => consts::OBXPropertyType_Byte,
        "i16" => consts::OBXPropertyType_Short,
        "u16" => consts::OBXPropertyType_Short,
        "char" => {
          println!("Warning: {} will be remapped behind the scenes as u32. A rusty char is 4 octets wide.", name);
          consts::OBXPropertyType_Char
        }, 
        "u32" => consts::OBXPropertyType_Int,
        "i32" => consts::OBXPropertyType_Int,
        "u64" => consts::OBXPropertyType_Long,
        "i64" => consts::OBXPropertyType_Long,
        "f32" => consts::OBXPropertyType_Float,
        "f64" => consts::OBXPropertyType_Double,
        "u8" => consts::OBXPropertyType_Byte,
        "String" => consts::OBXPropertyType_String,
        "VecString" => consts::OBXPropertyType_StringVector,
        "Vecu8" => consts::OBXPropertyType_ByteVector,
        _ => 0
      };

      *obx_property_flags |= match ident {
        "u8" => consts::OBXPropertyFlags_UNSIGNED,
        "u16" => consts::OBXPropertyFlags_UNSIGNED,
        "u32" => consts::OBXPropertyFlags_UNSIGNED,
        "u64" => consts::OBXPropertyFlags_UNSIGNED,
        _ => 0
      };
      
      return Some(property);
    }
    None
  }
}