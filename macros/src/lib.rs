extern crate maplit;
extern crate objectbox_generator;
extern crate prettyplease;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;
extern crate lean_buffer_internal;

use darling::FromDeriveInput;
use objectbox_generator::id;
use proc_macro::TokenStream;

mod debug;
mod entity;
mod path_visitor;
mod property;

use std::env;

use entity::Entity;
use syn::Meta::NameValue;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};

use std::str::FromStr;

use lean_buffer_internal::core::InputReceiver;

use std::path::{Path, PathBuf};

// extension trait for IdUid, reuse structs
trait IdUidMacroHelper {
    fn update_from_scan(&mut self, mnv: &syn::MetaNameValue);
    fn update_from_nested_metas(&mut self, iter: core::slice::Iter<syn::NestedMeta>);
}

impl IdUidMacroHelper for id::IdUid {
    fn update_from_scan(&mut self, mnv: &syn::MetaNameValue) {
        if let syn::Lit::Int(li) = &mnv.lit {
            let result = li.base10_parse::<u64>();
            if let Ok(value) = result {
                if let Some(ident) = mnv.path.get_ident() {
                    let param_name: &str = &ident.to_string();
                    match param_name {
                        "uid" => {
                            if self.uid == 0 {
                                self.uid = value
                            }
                        }
                        "id" => {
                            if self.id == 0 {
                                self.id = value
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn update_from_nested_metas(&mut self, iter: core::slice::Iter<syn::NestedMeta>) {
        iter.for_each(|nm| match nm {
            syn::NestedMeta::Meta(NameValue(mnv)) => {
                self.update_from_scan(mnv);
            }
            _ => {}
        });
    }
}

fn generate_fb_bindings(input: TokenStream) {
    let mut out = TokenStream::new();
    out.extend(TokenStream::from_str("#[derive(LeanBufferInternal)]"));
    out.extend(input);
    let parsed = syn::parse::<DeriveInput>(out).expect("crash");
    let mut receiver = InputReceiver::from_derive_input(&parsed).expect("crash");
    // receiver.write_raw(dest_path, Some("objectbox::traits".to_string()), Some("Factory".to_string()), Some("f".to_string()));
    receiver.write_raw_to_out_dir(
        Some("objectbox::traits".to_string()),
        Some("Factory".to_string()),
        Some("f".to_string()),
        // Some("f".to_string())
    );
}

fn _entity(input: TokenStream, args: Option<TokenStream>) -> TokenStream {
    // print_token_stream("all: ", input.clone());

    let out_dir = Path::new(&env::var_os("OUT_DIR").expect("OUT_DIR unknown")).to_path_buf();

    generate_fb_bindings(input.clone());

    let struct_clone = input.clone();
    // all parse_macro_input! macro have to happen inside a proc_macro_attribute(d) function
    let struct_info = parse_macro_input!(struct_clone as DeriveInput);

    let mut id = id::IdUid::zero();

    if args.is_some() {
        let args_unwrapped = args.unwrap();
        let attr_args = parse_macro_input!(args_unwrapped as AttributeArgs);
        if !attr_args.is_empty() {
            id.update_from_nested_metas(attr_args.iter());
        }
    }

    let entity = Entity::from_entity_name_and_fields(id, struct_info);
    entity.serialize().write(&out_dir);

    // dbg!(entity);

    input
        .into_iter()
        .map(|x| {
            if let proc_macro::TokenTree::Group(group) = x {
                let new_group = group
                    .stream()
                    .into_iter()
                    .filter(|y| {
                        match y {
                            // TODO make sure we only remove _our_ attributes (index, unique etc.)
                            // TODO replace false and '#' with something more intelligent
                            proc_macro::TokenTree::Group(_) => false,
                            proc_macro::TokenTree::Punct(p) => p.as_char() != '#',
                            _ => true,
                        }
                    })
                    .collect::<TokenStream>();
                let pm_group = proc_macro::Group::new(group.delimiter(), new_group);
                proc_macro::TokenTree::from(pm_group)
            } else {
                x
            }
        })
        .collect::<TokenStream>()
}

// This will break with nested sub types.
// The last bit will remove the annotations in the generated code
// because the generated code cannot reference the attributes.
// The result of this is unused imported attributes.
// TODO also remove those unused imports, in the generated code
#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    _entity(input, Some(args))
}

// TODO fix this, also allow for id/uid parameters to entity
// #[proc_macro_derive(OB, attributes(entity))]
// #[proc_macro_derive(OB, attributes(id, unique, index))]
// pub fn derive_ob_entity(input: TokenStream) -> TokenStream {
//     _entity(input, None)
//     // input
// }

#[proc_macro_attribute]
pub fn sync(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Fields

/// Accepts 'type' parameter
/// Note: indexes are currently not supported for ByteVector, Float or Double
/// See ./objectbox/lib/src/annotations.dart implementation
/// All ids are self-assignable, since there are no write-once / const fields.
#[proc_macro_attribute]
pub fn id(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn index(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Accepts 'uid'
#[proc_macro_attribute]
pub fn unique(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Accepts 'to'
#[proc_macro_attribute]
pub fn backlink(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// No required params
#[proc_macro_attribute]
pub fn transient(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Accepts 'type' and 'uid'
#[proc_macro_attribute]
pub fn property(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    input
}
