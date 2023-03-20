extern crate objectbox;

use example::{make_factory_map, make_model, Entity3};

use crate::objectbox::{opt::Opt, store::Store};

fn main() {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model).expect("crash");
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map).expect("crash");

    // box is a reserved keyword use r#box or simply something else
    let mut box1 = store.get_box::<Entity3>().expect("crash");

    let mut e_before = Entity3 {
        id: 0,
        hello: "Hello world!".to_string(),
    };

    let new_id = match box1.put(&mut e_before) {
        Err(err) => panic!("{err}"),
        Ok(item_id) => item_id,
    };

    match box1.get(new_id) {
        Err(err) => panic!("{err}"),
        Ok(found_item) => {
            if let Some(object) = found_item {
                println!("{}", object.hello);
            }
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use serial_test::serial;

    #[test]
    #[serial]
    fn test() {}
}
*/