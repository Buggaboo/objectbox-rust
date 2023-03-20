#![allow(dead_code)]
use std::marker::PhantomData;

use crate::c;

use lean_buffer::traits::{AdapterExt, FactoryExt};

pub trait IdExt {
    fn get_id(&self) -> c::obx_id;
    fn set_id(&mut self, id: c::obx_id);
}

// TODO
/*
pub trait RelationExt {
  fn to_one_relation<T>(&self) -> T;

  /// Any is rust's dynamic type? If so, then the relation's type
  /// requires the related trait.
  fn to_many_relations(&self) -> Any
}
*/

// Reference from Store and Box with this type
pub trait OBBlanket: IdExt + AdapterExt {}
impl<T> OBBlanket for T where T: IdExt + AdapterExt {}

use flatbuffers::Table;

pub trait EntityIdExt<T: ?Sized> {
    fn get_entity_id(&self) -> c::obx_schema_id;
}
pub trait FactoryBlanket<T>: EntityIdExt<T> + FactoryExt<T> {}
impl<T> FactoryBlanket<T> for T where T: EntityIdExt<T> + FactoryExt<T> {}

pub struct Factory<T> {
    pub phantom_data: PhantomData<T>,
    pub schema_id: c::obx_schema_id,
}

pub fn make_from_trait<T>(map: anymap::AnyMap, table: &mut Table) -> Option<T>
where
    T: 'static,
{
    if let Some(f) = map.get::<Box<dyn FactoryBlanket<T>>>() {
        return Some(f.inflate(table));
    }
    None
}

#[cfg(test)]
#[test]
fn blanket_directly_applied_on_entity_type() {
    // imagine this were an external struct
    // from a different package / crate / module etc.

    use crate::c;

    struct SomeEntity {
        id: c::obx_id,
    }

    impl AdapterExt for SomeEntity {
        fn flatten(&self, builder: &mut flatbuffers::FlatBufferBuilder) {}
    }

    impl IdExt for SomeEntity {
        fn get_id(&self) -> c::obx_id {
            self.id
        }
        fn set_id(&mut self, id: c::obx_id) {
            self.id = id;
        }
    }

    // call trait method on original object
    let e0 = SomeEntity { id: 1 };

    assert_eq!(e0.get_id(), 1);

    // single-owner boxed immutable
    let b1 = Box::new(SomeEntity { id: 3 });
    let t1 = b1 as Box<dyn OBBlanket>;

    assert_eq!(t1.get_id(), 3);

    // borrowed mutable
    let e2 = &mut SomeEntity { id: 5 };
    let m2 = e2 as &mut dyn OBBlanket;

    m2.set_id(5005);

    assert_eq!(m2.get_id(), 5005);

    // borrowed immutable
    let e3 = &SomeEntity { id: 6 };
    let r3 = e3 as &dyn OBBlanket;

    assert_eq!(r3.get_id(), 6);
}

#[cfg(test)]
#[test]
fn entity_factories() {
    unsafe {
        struct Entity0 {
            id: c::obx_schema_id,
        }
        struct Entity1 {
            id: c::obx_schema_id,
        }
        struct Entity2 {
            id: c::obx_schema_id,
        }

        impl EntityIdExt<Entity0> for Factory<Entity0> {
            fn get_entity_id(&self) -> c::obx_schema_id {
                0
            }
        }

        impl FactoryExt<Entity0> for Factory<Entity0> {
            fn inflate(&self, table: &mut Table) -> Entity0 {
                Entity0 { id: 0 }
            }
            fn new_object(&self) -> Entity0 {
                Entity0 { id: 0 }
            }
        }

        impl FactoryBlanket<Entity0> for Factory<Entity0> {}

        impl EntityIdExt<Entity1> for Factory<Entity1> {
            fn get_entity_id(&self) -> c::obx_schema_id {
                0
            }
        }

        impl FactoryExt<Entity1> for Factory<Entity1> {
            fn inflate(&self, table: &mut Table) -> Entity1 {
                Entity1 { id: 1 }
            }
            fn new_object(&self) -> Entity1 {
                Entity1 { id: 1 }
            }
        }

        impl EntityIdExt<Entity2> for Factory<Entity2> {
            fn get_entity_id(&self) -> c::obx_schema_id {
                2
            }
        }

        impl FactoryExt<Entity2> for Factory<Entity2> {
            fn inflate(&self, table: &mut Table) -> Entity2 {
                Entity2 { id: 2 }
            }
            fn new_object(&self) -> Entity2 {
                Entity2 { id: 2 }
            }
        }

        let table = &mut Table::new(&[0u8], 0);

        // this should be const boxed where it is generated
        let f0 = Factory::<Entity0> {
            phantom_data: PhantomData,
            schema_id: 1,
        };
        let f1 = Factory::<Entity1> {
            phantom_data: PhantomData,
            schema_id: 2,
        };
        let f2 = Factory::<Entity2> {
            phantom_data: PhantomData,
            schema_id: 3,
        };

        let e0 = f0.inflate(table);
        let e1 = f1.inflate(table);
        let e2 = f2.inflate(table);

        assert_eq!(e0.id, 0);
        assert_eq!(e1.id, 1);
        assert_eq!(e2.id, 2);

        // AnyMap experiment
        {
            let mut map = anymap::AnyMap::new();

            map.insert(f0);
            map.insert(f1);
            map.insert(f2);

            let f0 = map.get::<Factory<Entity0>>();
            let f1 = map.get::<Factory<Entity1>>();
            let f2 = map.get::<Factory<Entity2>>();

            let e0 = f0.unwrap().inflate(table);
            let e1 = f1.unwrap().inflate(table);
            let e2 = f2.unwrap().inflate(table);

            assert_eq!(e0.id, 0);
            assert_eq!(e1.id, 1);
            assert_eq!(e2.id, 2);
        }

        // experiment boxed factories
        {
            let mut map = anymap::AnyMap::new();
            let f0 = Factory::<Entity0> {
                phantom_data: PhantomData,
                schema_id: 0,
            };


            map.insert(Box::new(f0) as Box<dyn FactoryBlanket<Entity0>>);

            let e0 = make_from_trait::<Entity0>(map, table);
            assert_eq!(e0.is_some(), true); // \o/
        }

        // experiment ref'ed factories
        {
            fn make_from_ref<T>(map: anymap::AnyMap, table: &Table) -> Option<T>
            where
                T: 'static,
            {
                if let Some(f) = map.get::<Factory<T>>() {
                    // return f.inflate (nope, unknown trait)
                }
                None
            }

            let mut map = anymap::AnyMap::new();
            let f0: &'static Factory<Entity0> = &Factory::<Entity0> {
                phantom_data: PhantomData,
                schema_id: 0,
            };
            map.insert(f0);

            let e0 = make_from_ref::<Entity0>(map, table);
            assert_ne!(e0.is_some(), true); // :(
        }
    }
}
