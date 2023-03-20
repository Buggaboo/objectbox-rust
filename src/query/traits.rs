#![allow(dead_code)]
use crate::{c, traits::OBBlanket};
use core::marker::PhantomData;
use std::rc::Rc;

use super::{
    condition::{Condition, IdsAndType},
    enums::ConditionOp,
};

// Idea: lock down which ops are available given the generic param
// and the generated blanket.
// Collect the enums via the traits / blankets.
// Pass enums / tuples down to the builder.

pub fn create_condition_builder<
    T: OBBlanket,
    const ENTITY_ID: c::obx_schema_id,
    const PROPERTY_ID: c::obx_schema_id,
    const PROPERTY_TYPE: c::OBXPropertyType,
>() -> ConditionBuilder<T> {
    ConditionBuilder {
        phantom_data: PhantomData,
        ids_and_type: Rc::new((ENTITY_ID, PROPERTY_ID, PROPERTY_TYPE)),
    }
}

// Don't overcomplicate the generic params, because blankets
// depend on these, and it causes too much syntactical noise
pub struct ConditionBuilder<Entity: OBBlanket> {
    phantom_data: PhantomData<Entity>,
    // entity_id: obx_schema_id, property_id: obx_schema_id, property_type: u8,
    ids_and_type: IdsAndType,
}

impl<Entity: OBBlanket> ConditionBuilder<Entity> {
    fn get_property_attrs(&self) -> IdsAndType {
        self.ids_and_type.clone()
    }
}

pub trait BasicExt<Entity: OBBlanket> {
    fn order_flags(&mut self, of: u32) -> Condition<Entity>;

    // TODO test when there is support for Option<*> properties

    fn is_null(&self) -> Condition<Entity>;
    fn is_not_null(&self) -> Condition<Entity>;
}

impl<Entity: OBBlanket> BasicExt<Entity> for ConditionBuilder<Entity> {
    fn order_flags(&mut self, of: u32) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::OrderFlags(of))
    }

    // TODO test when there is support for Option<*> properties
    fn is_null(&self) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::IsNull)
    }
    fn is_not_null(&self) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::NotNull)
    }
}

// TODO figure out if std::ops really doesn't contain <, >, <=, >=
// If op overloading has to be thru, the std::cmp::Partial{Ord,Eq}
// then no op overloading, Because every op return type is bool.
pub trait EqExt<Entity: OBBlanket, Rhs>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: Rhs) -> Condition<Entity>;
    fn ne(&self, other: Rhs) -> Condition<Entity>;
}

pub trait OrdExt<Entity: OBBlanket, Rhs>
where
    Rhs: ?Sized,
{
    fn lt(&self, other: Rhs) -> Condition<Entity>;
    fn gt(&self, other: Rhs) -> Condition<Entity>;
    fn le(&self, other: Rhs) -> Condition<Entity>;
    fn ge(&self, other: Rhs) -> Condition<Entity>;
}

pub trait StringExt<Entity: OBBlanket> {
    fn contains(&self, s: &str) -> Condition<Entity>;
    fn contains_element(&self, s: &str) -> Condition<Entity>;
    fn contains_key_value(&self, start: &str, end: &str) -> Condition<Entity>;
    fn starts_with(&self, s: &str) -> Condition<Entity>;
    fn ends_with(&self, s: &str) -> Condition<Entity>;
    fn in_strings(&self, vec: &Vec<String>) -> Condition<Entity>;
    fn any_equals(&self, list: &str) -> Condition<Entity>; // not sure about the input type
    fn case_sensitive(&self, b: bool) -> Condition<Entity>;
}

impl<Entity: OBBlanket> StringExt<Entity> for ConditionBuilder<Entity> {
    fn contains(&self, s: &str) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::Contains(s.to_string()),
        )
    }

    fn contains_element(&self, s: &str) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::ContainsElement(s.to_string()),
        )
    }

    fn starts_with(&self, s: &str) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::StartsWith(s.to_string()),
        )
    }

    fn ends_with(&self, s: &str) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::EndsWith(s.to_string()),
        )
    }

    fn any_equals(&self, s: &str) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::AnyEquals(s.to_string()),
        )
    }

    /// The entire query will become case sensitive
    fn case_sensitive(&self, b: bool) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::CaseSensitive(b))
    }

    fn contains_key_value(&self, start: &str, end: &str) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::ContainsKeyValue(String::from(start), String::from(end)),
        )
    }

    fn in_strings(&self, vec: &Vec<String>) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::In_String(vec.to_vec()),
        )
    }
}

pub trait BetweenExt<Entity: OBBlanket, SurroundType>
where
    SurroundType: ?Sized,
{
    fn between(&self, this: SurroundType, that: SurroundType) -> Condition<Entity>;
}

pub trait InOutExt<Entity: OBBlanket, U>
where
    U: Sized,
{
    fn member_of(&self, vec: Vec<U>) -> Condition<Entity>;
    fn not_member_of(&self, vec: Vec<U>) -> Condition<Entity>;
}

impl<Entity: OBBlanket> EqExt<Entity, i64> for ConditionBuilder<Entity> {
    fn eq(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Eq_i64(other))
    }
    fn ne(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ne_i64(other))
    }
}

impl<Entity: OBBlanket> OrdExt<Entity, i64> for ConditionBuilder<Entity> {
    fn lt(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Lt_i64(other))
    }
    fn gt(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Gt_i64(other))
    }
    fn le(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Le_i64(other))
    }
    fn ge(&self, other: i64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ge_i64(other))
    }
}

impl<Entity: OBBlanket> OrdExt<Entity, f64> for ConditionBuilder<Entity> {
    fn lt(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Lt_f64(other))
    }
    fn gt(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Gt_f64(other))
    }
    fn le(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Le_f64(other))
    }
    fn ge(&self, other: f64) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ge_f64(other))
    }
}

impl<Entity: OBBlanket> EqExt<Entity, String> for ConditionBuilder<Entity> {
    fn eq(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Eq_string(other))
    }
    fn ne(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ne_string(other))
    }
}

impl<Entity: OBBlanket> OrdExt<Entity, String> for ConditionBuilder<Entity> {
    fn lt(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Lt_string(other))
    }
    fn gt(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Gt_string(other))
    }
    fn le(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Le_string(other))
    }
    fn ge(&self, other: String) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ge_string(other))
    }
}

impl<Entity: OBBlanket> EqExt<Entity, Vec<u8>> for ConditionBuilder<Entity> {
    fn eq(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Eq_vecu8(other))
    }
    fn ne(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::NoOp)
    }
}

impl<Entity: OBBlanket> OrdExt<Entity, Vec<u8>> for ConditionBuilder<Entity> {
    fn lt(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Lt_vecu8(other))
    }
    fn gt(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Gt_vecu8(other))
    }
    fn le(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Le_vecu8(other))
    }
    fn ge(&self, other: Vec<u8>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::Ge_vecu8(other))
    }
}

impl<Entity: OBBlanket> BetweenExt<Entity, i64> for ConditionBuilder<Entity> {
    fn between(&self, this: i64, that: i64) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::Between_i64(this, that),
        )
    }
}
impl<Entity: OBBlanket> BetweenExt<Entity, f64> for ConditionBuilder<Entity> {
    fn between(&self, this: f64, that: f64) -> Condition<Entity> {
        Condition::new(
            self.get_property_attrs(),
            ConditionOp::Between_f64(this, that),
        )
    }
}
impl<Entity: OBBlanket> InOutExt<Entity, i32> for ConditionBuilder<Entity> {
    fn member_of(&self, vec: Vec<i32>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::In_i32(vec))
    }
    fn not_member_of(&self, vec: Vec<i32>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::NotIn_i32(vec))
    }
}
impl<Entity: OBBlanket> InOutExt<Entity, i64> for ConditionBuilder<Entity> {
    fn member_of(&self, vec: Vec<i64>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::In_i64(vec))
    }
    fn not_member_of(&self, vec: Vec<i64>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::NotIn_i64(vec))
    }
}
impl<Entity: OBBlanket> InOutExt<Entity, String> for ConditionBuilder<Entity> {
    fn member_of(&self, vec: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::In_String(vec))
    }
    fn not_member_of(&self, vec: Vec<String>) -> Condition<Entity> {
        Condition::new(self.get_property_attrs(), ConditionOp::NoOp)
    }
}

/// Blankets
pub trait BoolBlanket<Entity: OBBlanket>: BasicExt<Entity> // + EqExt<Entity, i64>
{
}

pub trait CharBlanket<Entity: OBBlanket>:
    BasicExt<Entity>
    + EqExt<Entity, i64>
    + OrdExt<Entity, i64>
    + BetweenExt<Entity, i64>
    + InOutExt<Entity, i32>
{
}

pub trait I8Blanket<Entity: OBBlanket>:
    BasicExt<Entity>
    + EqExt<Entity, i64>
    + OrdExt<Entity, i64>
    + BetweenExt<Entity, i64>
    + InOutExt<Entity, i32>
{
}

pub trait U8Blanket<Entity: OBBlanket>:
    BasicExt<Entity>
    + EqExt<Entity, i64>
    + OrdExt<Entity, i64>
    + BetweenExt<Entity, i64>
    + InOutExt<Entity, i32>
{
}

pub trait F32Blanket<Entity: OBBlanket>:
    BasicExt<Entity> + OrdExt<Entity, f64> + BetweenExt<Entity, f64>
{
}
pub trait F64Blanket<Entity: OBBlanket>:
    BasicExt<Entity> + OrdExt<Entity, f64> + BetweenExt<Entity, f64>
{
}

pub trait I16Blanket<Entity: OBBlanket>:
    BasicExt<Entity>
    + EqExt<Entity, i64>
    + OrdExt<Entity, i64>
    + BetweenExt<Entity, i64>
    + InOutExt<Entity, i32>
{
}
pub trait U16Blanket<Entity: OBBlanket>:
    BasicExt<Entity>
    + EqExt<Entity, i64>
    + OrdExt<Entity, i64>
    + BetweenExt<Entity, i64>
    + InOutExt<Entity, i32>
{
}

pub trait I32Blanket<Entity: OBBlanket>:
    BasicExt<Entity>
    + EqExt<Entity, i64>
    + OrdExt<Entity, i64>
    + BetweenExt<Entity, i64>
    + InOutExt<Entity, i32>
{
}
pub trait U32Blanket<Entity: OBBlanket>:
    BasicExt<Entity>
    + EqExt<Entity, i64>
    + OrdExt<Entity, i64>
    + BetweenExt<Entity, i64>
    + InOutExt<Entity, i32>
{
}

pub trait I64Blanket<Entity: OBBlanket>:
    BasicExt<Entity>
    + EqExt<Entity, i64>
    + OrdExt<Entity, i64>
    + BetweenExt<Entity, i64>
    + InOutExt<Entity, i64>
{
}
pub trait U64Blanket<Entity: OBBlanket>:
    BasicExt<Entity>
    + EqExt<Entity, i64>
    + OrdExt<Entity, i64>
    + BetweenExt<Entity, i64>
    + InOutExt<Entity, i64>
{
}

pub trait VecU8Blanket<Entity: OBBlanket>:
    BasicExt<Entity> + EqExt<Entity, Vec<u8>> + OrdExt<Entity, Vec<u8>>
{
}

pub trait StringBlanket<Entity: OBBlanket>:
    StringExt<Entity>
    + BasicExt<Entity>
    + EqExt<Entity, String>
    + OrdExt<Entity, String>
    + InOutExt<Entity, String>
{
}

impl<Entity: OBBlanket> F32Blanket<Entity> for Entity where
    Entity: BasicExt<Entity> + OrdExt<Entity, f64> + BetweenExt<Entity, f64>
{
}
impl<Entity: OBBlanket> F64Blanket<Entity> for Entity where
    Entity: BasicExt<Entity> + OrdExt<Entity, f64> + BetweenExt<Entity, f64>
{
}
impl<Entity: OBBlanket> BoolBlanket<Entity> for Entity where
    Entity: BasicExt<Entity> // + EqExt<Entity, i64>
{
}
impl<Entity: OBBlanket> CharBlanket<Entity> for Entity where
    Entity: BasicExt<Entity>
        + EqExt<Entity, i64>
        + OrdExt<Entity, i64>
        + BetweenExt<Entity, i64>
        + InOutExt<Entity, i32>
{
}
impl<Entity: OBBlanket> I8Blanket<Entity> for Entity where
    Entity: BasicExt<Entity>
        + EqExt<Entity, i64>
        + OrdExt<Entity, i64>
        + BetweenExt<Entity, i64>
        + InOutExt<Entity, i32>
{
}
impl<Entity: OBBlanket> U8Blanket<Entity> for Entity where
    Entity: BasicExt<Entity>
        + EqExt<Entity, i64>
        + OrdExt<Entity, i64>
        + BetweenExt<Entity, i64>
        + InOutExt<Entity, i32>
{
}
impl<Entity: OBBlanket> I16Blanket<Entity> for Entity where
    Entity: BasicExt<Entity>
        + EqExt<Entity, i64>
        + OrdExt<Entity, i64>
        + BetweenExt<Entity, i64>
        + InOutExt<Entity, i32>
{
}
impl<Entity: OBBlanket> U16Blanket<Entity> for Entity where
    Entity: BasicExt<Entity>
        + EqExt<Entity, i64>
        + OrdExt<Entity, i64>
        + BetweenExt<Entity, i64>
        + InOutExt<Entity, i32>
{
}
impl<Entity: OBBlanket> I32Blanket<Entity> for Entity where
    Entity: BasicExt<Entity>
        + EqExt<Entity, i64>
        + OrdExt<Entity, i64>
        + BetweenExt<Entity, i64>
        + InOutExt<Entity, i32>
{
}
impl<Entity: OBBlanket> U32Blanket<Entity> for Entity where
    Entity: BasicExt<Entity>
        + EqExt<Entity, i64>
        + OrdExt<Entity, i64>
        + BetweenExt<Entity, i64>
        + InOutExt<Entity, i32>
{
}
impl<Entity: OBBlanket> I64Blanket<Entity> for Entity where
    Entity: BasicExt<Entity>
        + EqExt<Entity, i64>
        + OrdExt<Entity, i64>
        + BetweenExt<Entity, i64>
        + InOutExt<Entity, i64>
{
}
impl<Entity: OBBlanket> U64Blanket<Entity> for Entity where
    Entity: BasicExt<Entity>
        + EqExt<Entity, i64>
        + OrdExt<Entity, i64>
        + BetweenExt<Entity, i64>
        + InOutExt<Entity, i64>
{
}
impl<Entity: OBBlanket> VecU8Blanket<Entity> for Entity where
    Entity: BasicExt<Entity> + EqExt<Entity, Vec<u8>> + OrdExt<Entity, Vec<u8>>
{
}
impl<Entity: OBBlanket> StringBlanket<Entity> for Entity where
    Entity: StringExt<Entity>
        + BasicExt<Entity>
        + EqExt<Entity, String>
        + OrdExt<Entity, String>
        + InOutExt<Entity, String>
{
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use lean_buffer::traits::AdapterExt;

    use crate::traits;

    use super::*;

    struct TEntity {
        id: u64,
    }

    struct TEntity2 {
        id: u64,
    }

    impl traits::IdExt for TEntity {
        fn get_id(&self) -> c::obx_id {
            0
        }
        fn set_id(&mut self, id: c::obx_id) {}
    }

    impl AdapterExt for TEntity {
        fn flatten(&self, builder: &mut flatbuffers::FlatBufferBuilder) {}
    }

    impl traits::IdExt for TEntity2 {
        fn get_id(&self) -> c::obx_id {
            2
        }
        fn set_id(&mut self, id: c::obx_id) {}
    }

    impl AdapterExt for TEntity2 {
        fn flatten(&self, builder: &mut flatbuffers::FlatBufferBuilder) {}
    }

    // conflicts with original generic one
    // impl traits::OBBlanket for TEntity2 {}

    struct EntityConditionFactories<'a> {
        foxtrot: &'a dyn F64Blanket<TEntity2>,
        charlie: &'a dyn F64Blanket<TEntity2>,
    }

    struct EntityConditionFactories2 {
        uniform: ConditionBuilder<TEntity2>,
        kilo: ConditionBuilder<TEntity2>,
    }

    fn group_condition_builders() -> EntityConditionFactories2 {
        EntityConditionFactories2 {
            uniform: create_condition_builder::<TEntity2, 1, 1, 1>(), // as &dyn F64Blanket<TEntity2>,
            kilo: create_condition_builder::<TEntity2, 1, 1, 1>(), // as &dyn F64Blanket<TEntity2>,
        }
    }

    impl I16Blanket<TEntity2> for ConditionBuilder<TEntity2> {}
    impl I64Blanket<TEntity2> for ConditionBuilder<TEntity2> {}
    impl BoolBlanket<TEntity2> for ConditionBuilder<TEntity2> {}
    impl I32Blanket<TEntity2> for ConditionBuilder<TEntity2> {}
    impl I8Blanket<TEntity2> for ConditionBuilder<TEntity2> {}
    impl F64Blanket<TEntity2> for ConditionBuilder<TEntity2> {}
    impl VecU8Blanket<TEntity2> for ConditionBuilder<TEntity2> {}
    impl CharBlanket<TEntity2> for ConditionBuilder<TEntity2> {}
    impl StringBlanket<TEntity2> for ConditionBuilder<TEntity2> {}
    impl F32Blanket<TEntity2> for ConditionBuilder<TEntity2> {}

    #[test]
    fn trait_impl_test() {
        use std::rc::Rc;
        let cb1: ConditionBuilder<TEntity> = ConditionBuilder {
            phantom_data: PhantomData,
            ids_and_type: Rc::new((1, 1, 1)),
        };

        let mut cb2: ConditionBuilder<TEntity2> = ConditionBuilder {
            phantom_data: PhantomData,
            ids_and_type: Rc::new((2, 2, 2)),
        };

        let boxed_cb1 = Box::new(cb1);
        let mock_condition1 = boxed_cb1.ge(0.0); // works, then F32 and F64 make it ambiguous
        let mock_condition2 = boxed_cb1.ge(0); // works, then I* and U* make it ambiguous

        // The  following lines need to be generated:
        let _ = HashMap::<String, Box<dyn std::any::Any>>::new();
        // impl F64Blanket<TEntity2> for ConditionBuilder<TEntity2> {}

        // map.insert("name_of_field".to_string(), Box::<dyn F64Blanket<TEntity2>>::new(create_condition_builder::<TEntity2, 3, 3, 3>() as dyn F64Blanket<TEntity2>));

        // works
        let cb3 = &create_condition_builder::<TEntity2, 3, 3, 3>() as &dyn F64Blanket<TEntity2>;

        // let retype_cb2: &dyn F64Blanket<TEntity2> = &cb2 as &dyn F64Blanket<TEntity2>;
        let retype_cb2 = &cb2 as &dyn F64Blanket<TEntity2>;
        let _ = retype_cb2.between(0.000000000001, 2.0000000000000000000);

        let _ = EntityConditionFactories {
            foxtrot: &create_condition_builder::<TEntity2, 1, 1, 1>() as &dyn F64Blanket<TEntity2>,
            charlie: &cb2 as &dyn F64Blanket<TEntity2>,
        };
        let _ = EntityConditionFactories {
            foxtrot: retype_cb2,
            charlie: retype_cb2,
        };

        // Correct: Compile error, between belongs in a different table
        // mock_condition1.and(between_cond).or(mock_condition2);

        // Yes, same table
        mock_condition1.and(mock_condition2);
        let _ = &mut cb2.order_flags(1);

        boxed_cb1.is_not_null(); // basic op, all properties should be capable of doing this check

        let g = group_condition_builders();
    }

    struct EntityConditionFactory {
        id: Box<dyn StringBlanket<TEntity2>>,
    }
    fn new_entity_condition_factory() -> EntityConditionFactory {
        EntityConditionFactory {
            id: Box::new(create_condition_builder::<TEntity2, 1, 1, 6>()), /*as Box<&dyn I64Blanket<TEntity2>>*/
        }
    }

    #[test]
    fn trait_impl_test2() {
        let necf = new_entity_condition_factory();
        necf.id.contains("stuff");
    }
}
