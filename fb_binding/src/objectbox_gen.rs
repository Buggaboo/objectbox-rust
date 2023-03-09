use objectbox::c;
use objectbox::flatbuffers;
use objectbox::map;
use objectbox::model;
use objectbox::query::traits as qtraits;
use objectbox::traits;
use std::marker;
use std::rc;
impl traits::IdExt for crate::Entity {
    fn get_id(&self) -> c::obx_id {
        self.id
    }
    fn set_id(&mut self, id: c::obx_id) {
        self.id = id;
    }
}
impl traits::FBOBBridge for crate::Entity {
    fn to_fb(&self, builder: &mut flatbuffers::FlatBufferBuilder) {
        builder.reset();
        let str_28 = builder.create_string(self.t_string.as_str());
        let byte_vec_30 = builder.create_vector(&self.t_vec_u8.as_slice());
        let strs_vec_32 = self
            .t_vec_string
            .iter()
            .map(|s| builder.create_string(s.as_str()))
            .collect::<Vec<flatbuffers::WIPOffset<&str>>>();
        let vec_32 = builder.create_vector(strs_vec_32.as_slice());
        let wip_offset_unfinished = builder.start_table();
        builder.push_slot::<u64>(4, self.id, 0);
        builder.push_slot::<u64>(22, self.t_u64, 0);
        builder.push_slot::<i64>(24, self.t_i64, 0);
        builder.push_slot::<f64>(26, self.t_f64, 0.0);
        builder.push_slot_always(32, vec_32);
        builder.push_slot_always(30, byte_vec_30);
        builder.push_slot_always(28, str_28);
        builder.push_slot::<i32>(16, self.t_i32, 0);
        builder.push_slot::<u32>(18, self.t_u32, 0);
        builder.push_slot::<f32>(20, self.t_f32, 0.0);
        builder.push_slot::<i16>(12, self.t_i16, 0);
        builder.push_slot::<u16>(14, self.t_u16, 0);
        builder.push_slot::<i8>(6, self.t_i8, 0);
        builder.push_slot::<u8>(8, self.t_u8, 0);
        builder.push_slot::<bool>(10, self.t_bool, false);
        let wip_offset_finished = builder.end_table(wip_offset_unfinished);
        builder.finish_minimal(wip_offset_finished);
    }
}
impl traits::EntityFactoryExt<crate::Entity> for traits::Factory<crate::Entity> {
    fn make(&self, table: &mut flatbuffers::Table) -> crate::Entity {
        let mut object = self.new_entity();
        let crate::Entity {
            id,
            t_i8,
            t_u8,
            t_bool,
            t_i16,
            t_u16,
            t_i32,
            t_u32,
            t_f32,
            t_u64,
            t_i64,
            t_f64,
            t_string,
            t_vec_u8,
            t_vec_string,
        } = &mut object;
        unsafe {
            *id = table.get::<u64>(4, Some(0)).unwrap();
            *t_i8 = table.get::<i8>(6, Some(0)).unwrap();
            *t_u8 = table.get::<u8>(8, Some(0)).unwrap();
            *t_bool = table.get::<bool>(10, Some(false)).unwrap();
            *t_i16 = table.get::<i16>(12, Some(0)).unwrap();
            *t_u16 = table.get::<u16>(14, Some(0)).unwrap();
            *t_i32 = table.get::<i32>(16, Some(0)).unwrap();
            *t_u32 = table.get::<u32>(18, Some(0)).unwrap();
            *t_f32 = table.get::<f32>(20, Some(0.0)).unwrap();
            *t_u64 = table.get::<u64>(22, Some(0)).unwrap();
            *t_i64 = table.get::<i64>(24, Some(0)).unwrap();
            *t_f64 = table.get::<f64>(26, Some(0.0)).unwrap();
            if let Some(s) = table.get::<flatbuffers::ForwardsUOffset<&str>>(28, None) {
                *t_string = s.to_string();
            }
            let fb_vec_t_vec_u8 = table
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<u8>>>(30, None);
            if let Some(bv) = fb_vec_t_vec_u8 {
                *t_vec_u8 = bv.bytes().to_vec();
            }
            let fb_vec_t_vec_string = table
                .get::<
                    flatbuffers::ForwardsUOffset<
                        flatbuffers::Vector<flatbuffers::ForwardsUOffset<&str>>,
                    >,
                >(32, None);
            if let Some(sv) = fb_vec_t_vec_string {
                *t_vec_string = sv.iter().map(|s| s.to_string()).collect();
            }
        }
        object
    }
    fn get_entity_id(&self) -> c::obx_schema_id {
        self.schema_id
    }
    fn new_entity(&self) -> crate::Entity {
        crate::Entity {
            id: 0,
            t_i8: 0,
            t_u8: 0,
            t_bool: false,
            t_i16: 0,
            t_u16: 0,
            t_i32: 0,
            t_u32: 0,
            t_f32: 0.0,
            t_u64: 0,
            t_i64: 0,
            t_f64: 0.0,
            t_string: String::from(""),
            t_vec_u8: Vec::<u8>::new(),
            t_vec_string: Vec::<String>::new(),
        }
    }
}
impl qtraits::F32Blanket<crate::Entity> for qtraits::ConditionBuilder<crate::Entity> {}
impl qtraits::VecU8Blanket<crate::Entity> for qtraits::ConditionBuilder<crate::Entity> {}
impl qtraits::BoolBlanket<crate::Entity> for qtraits::ConditionBuilder<crate::Entity> {}
impl qtraits::I8Blanket<crate::Entity> for qtraits::ConditionBuilder<crate::Entity> {}
impl qtraits::I16Blanket<crate::Entity> for qtraits::ConditionBuilder<crate::Entity> {}
impl qtraits::I32Blanket<crate::Entity> for qtraits::ConditionBuilder<crate::Entity> {}
impl qtraits::I64Blanket<crate::Entity> for qtraits::ConditionBuilder<crate::Entity> {}
impl qtraits::F64Blanket<crate::Entity> for qtraits::ConditionBuilder<crate::Entity> {}
impl qtraits::StringBlanket<crate::Entity> for qtraits::ConditionBuilder<crate::Entity> {}
pub struct EntityConditionFactory {
    pub id: Box<dyn qtraits::I64Blanket<crate::Entity>>,
    pub t_i8: Box<dyn qtraits::I8Blanket<crate::Entity>>,
    pub t_u8: Box<dyn qtraits::I8Blanket<crate::Entity>>,
    pub t_bool: Box<dyn qtraits::BoolBlanket<crate::Entity>>,
    pub t_i16: Box<dyn qtraits::I16Blanket<crate::Entity>>,
    pub t_u16: Box<dyn qtraits::I16Blanket<crate::Entity>>,
    pub t_i32: Box<dyn qtraits::I32Blanket<crate::Entity>>,
    pub t_u32: Box<dyn qtraits::I32Blanket<crate::Entity>>,
    pub t_f32: Box<dyn qtraits::F32Blanket<crate::Entity>>,
    pub t_u64: Box<dyn qtraits::I64Blanket<crate::Entity>>,
    pub t_i64: Box<dyn qtraits::I64Blanket<crate::Entity>>,
    pub t_f64: Box<dyn qtraits::F64Blanket<crate::Entity>>,
    pub t_string: Box<dyn qtraits::StringBlanket<crate::Entity>>,
    pub t_vec_u8: Box<dyn qtraits::VecU8Blanket<crate::Entity>>,
}
pub fn new_entity_condition_factory() -> EntityConditionFactory {
    EntityConditionFactory {
        id: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 1, 6>()),
        t_i8: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 2, 2>()),
        t_u8: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 3, 2>()),
        t_bool: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 4, 1>()),
        t_i16: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 5, 3>()),
        t_u16: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 6, 3>()),
        t_i32: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 7, 5>()),
        t_u32: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 8, 5>()),
        t_f32: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 9, 7>()),
        t_u64: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 10, 6>()),
        t_i64: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 11, 6>()),
        t_f64: Box::new(qtraits::create_condition_builder::<crate::Entity, 1, 12, 8>()),
        t_string: Box::new(
            qtraits::create_condition_builder::<crate::Entity, 1, 13, 9>(),
        ),
        t_vec_u8: Box::new(
            qtraits::create_condition_builder::<crate::Entity, 1, 14, 23>(),
        ),
    }
}
pub fn make_model() -> model::Model {
    model::Model::new()
        .entity("Entity", 1, 6934213297317435850)
        .property("id", 1, 7398129845820662226, 6, 129)
        .property("t_u64", 10, 6762680243245672799, 6, 8192)
        .property("t_i64", 11, 3412405623712805883, 6, 0)
        .property("t_f64", 12, 4525049858390567096, 8, 0)
        .property("t_vec_string", 15, 12538710810757874575, 30, 0)
        .property("t_vec_u8", 14, 8696023648830558810, 23, 0)
        .property("t_string", 13, 8100121304458598589, 9, 0)
        .property("t_i32", 7, 6638210980164788377, 5, 0)
        .property("t_u32", 8, 14713831257541507981, 5, 8192)
        .property("t_f32", 9, 17998607689778292859, 7, 0)
        .property("t_i16", 5, 8650354029225646382, 3, 0)
        .property("t_u16", 6, 11832362041913486637, 3, 8192)
        .property("t_i8", 2, 16937435693226446575, 2, 0)
        .property("t_u8", 3, 11996711052919561065, 2, 8192)
        .property("t_bool", 4, 4285115186600216247, 1, 0)
        .last_property_id(15, 12538710810757874575)
        .last_entity_id(1, 6934213297317435850)
}
pub fn make_factory_map() -> map::AnyMap {
    let mut map = map::AnyMap::new();
    let f1 = rc::Rc::new(traits::Factory::<crate::Entity> {
        phantom_data: marker::PhantomData,
        schema_id: 1,
    }) as rc::Rc<dyn traits::EntityFactoryExt<crate::Entity>>;
    map.insert(f1);
    map
}
