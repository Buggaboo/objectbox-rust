#![allow(dead_code)]
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr};
use std::rc::Rc;

use crate::c::{self, obx_schema_id};
use crate::query::enums::ConditionOp;
use crate::traits::OBBlanket;
use crate::util::QUERY_NO_OP;

/// entity id, property id, property type
pub type IdsAndType = Rc<(obx_schema_id, obx_schema_id, c::OBXPropertyType)>;
/// All conditions are collected then passed on to a QueryBuilder

pub struct Condition<Entity: OBBlanket> {
    phantom_data: PhantomData<Entity>,
    ids_and_type: IdsAndType,
    pub(crate) op: ConditionOp,
    // This could have been a parameter the All/Any enum type
    // but it introduced syntactical noise due to generics
    // to the other enum values. Now we have a (directional) tree.
    pub(crate) group: Option<Vec<Self>>,
    pub(crate) result: Option<c::obx_qb_cond>,
}

impl<Entity: OBBlanket> Condition<Entity> {
    pub(crate) fn get_property_id(&self) -> c::obx_schema_id {
        self.ids_and_type.1
    }

    pub(crate) fn get_entity_id(&self) -> c::obx_schema_id {
        self.ids_and_type.0
    }

    pub(crate) fn new_group(ids_and_type: IdsAndType, op: ConditionOp, group: Vec<Self>) -> Self {
        Self {
            phantom_data: PhantomData,
            ids_and_type,
            op,
            group: Some(group),
            result: None,
        }
    }

    pub(crate) fn new(ids_and_type: IdsAndType, op: ConditionOp) -> Self {
        Self {
            phantom_data: PhantomData,
            ids_and_type,
            op,
            group: None,
            result: None,
        }
    }

    pub fn or(self, that: Condition<Entity>) -> Self {
        Self::new_group(
            self.ids_and_type.clone(),
            ConditionOp::Any,
            vec![self, that],
        )
    }

    pub fn and(self, that: Condition<Entity>) -> Self {
        Self::new_group(
            self.ids_and_type.clone(),
            ConditionOp::All,
            vec![self, that],
        )
    }

    pub fn or_any(self, mut those: Vec<Condition<Entity>>) -> Self {
        let id_t = self.ids_and_type.clone();
        those.insert(0, self);
        Self::new_group(id_t, ConditionOp::Any, those)
    }

    pub fn and_all(self, mut those: Vec<Condition<Entity>>) -> Self {
        let id_t = self.ids_and_type.clone();
        those.insert(0, self);
        Self::new_group(id_t, ConditionOp::All, those)
    }

    pub(crate) fn collect_results(&self) -> Vec<c::obx_qb_cond> {
        let mut vec = Vec::<c::obx_qb_cond>::new();
        if let Some(children) = &self.group {
            for c in children {
                if let Some(r) = c.result {
                    vec.push(r);
                }
            }
        }
        vec
    }

    pub(crate) fn visit_dfs(&mut self, f: &mut impl FnMut(&mut Self) -> c::obx_qb_cond) {
        if let Some(cs) = &mut self.group {
            for c in cs {
                c.visit_dfs(f)
            }
        }
        let i = f(self);
        self.result = if i == QUERY_NO_OP { None } else { Some(i) }
    }
}

impl<Entity: OBBlanket> BitAnd for Condition<Entity> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl<Entity: OBBlanket> BitOr for Condition<Entity> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

#[cfg(test)]
mod tests {
    use flatbuffers::FlatBufferBuilder;
    use lean_buffer::traits::AdapterExt;

    use super::*;

    use crate::{
        c,
        traits::IdExt,
    };

    struct SomeEntity {
        id: c::obx_id,
        t_string: String,
    }

    impl AdapterExt for SomeEntity {
        fn flatten(&self, _: &mut FlatBufferBuilder<'_>) {}
    }

    impl IdExt for SomeEntity {
        fn get_id(&self) -> c::obx_id {
            self.id
        }
        fn set_id(&mut self, id: c::obx_id) {
            self.id = id;
        }
    }

    #[test]
    fn query_bitandor_overload() {
        let it = IdsAndType::new((0, 1, 2));
        let c1 =
            Condition::<SomeEntity>::new(it.clone(), ConditionOp::EndsWith("1234".to_string()));
        let c2 =
            Condition::<SomeEntity>::new(it.clone(), ConditionOp::StartsWith("1234".to_string()));
        let c3 =
            Condition::<SomeEntity>::new(it.clone(), ConditionOp::EndsWith("1234".to_string()));
        let c4 =
            Condition::<SomeEntity>::new(it.clone(), ConditionOp::StartsWith("1234".to_string()));

        let _ = c1 | c2;
        let _ = c3 & c4;
    }
}
