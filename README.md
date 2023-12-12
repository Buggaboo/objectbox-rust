# Objectbox for rust
Unofficial rust support for ObjectBox database.

See the [example crate](example/src/main.rs).

## Example
```rust
extern crate objectbox;

use objectbox::{macros::entity, opt::Opt, store::Store};

mod objectbox_gen;
use objectbox_gen as ob;

#[derive(Debug)]
#[entity]
pub struct Entity {
    #[id]
    id: u64,
    hello: String,
}

fn main() {
    let mut model = ob::make_model();
    let opt = Opt::from_model(&mut model);
    let trait_map = ob::make_factory_map();
    let store = Store::new(opt, trait_map).expect("crash");

    let mut box1 = store.get_box::<Entity>().expect("crash");

    let mut e_before = Entity {
        id: 0,
        hello: "Hello world!".to_string(),
    };

    let new_id = box1.put(&mut e_before).expect("crash");

    match box1.get(new_id) {
        Err(err) => panic!("{err}"),
        Ok(found_item) => {
            if let Some(object) = found_item {
                println!("{}", object.hello);
            }
        }
    }
}
```

## How the packages cooperate
### [The macros package](macros/src/lib.rs)
This is where the rust meta attributes are defined to parse structs, that triggers
the build to produce files with the '.objectbox.info' suffix.

### [The generator package](generator/src/lib.rs)
Together, with the [build.rs](example/build.rs) file, the `entity.objectbox.info` files
are globbed and processed, to generate a `objectbox-model.json` file.

In the final stage, `objectbox-model.json` is used to generate all the necessary
rust code to facilitate and access the basic and/or advanced features, in `objectbox_gen.rs`.

## Dependencies
* InstallRustup, or get it from apt, brew, chocolatey, etc.
* llvm
* Also, make sure llvm-ar is also exported in `$PATH`

## Abstract roadmap
* Fix String Query bugs, also write more tests
* Support fields with `Option<P>` where `P` is some primitive type
* Write more tests, especially for all condition ops

## Problems solved, 2023 Feb-March
* Testing query conditions implementation
* Code generation from struct entities with macros
* Code generation for injecting the model to Store
* Weave traits to make blankets, so objects can be created, flattened, inflated.

## Guidelines
* Don't rely on nightly features, we'll take whatever edition 2021 has to offer

## TODO
* Apply `Arc<str>` for const-ish strs
* Reduce number of mallocs
* Use [anyhow](https://docs.rs/anyhow/latest/anyhow/) to handle errors
* Remove cruft - all attempts to optimize prematurely
* Reimplement macros with [darling's](https://github.com/TedDriggs/darling/blob/master/examples/consume_fields.rs)
* Integrate: [cleaner abstractions (example how)](https://github.com/Buggaboo/lean_buffer/blob/main/macros/src/lib.rs).
