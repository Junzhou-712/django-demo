use vec_map::VecMap;
use self::constant_pool::ConstantPool;
use self::interface_info::InterfaceInfo;
use self::field_info::FieldInfo;
use self::method_info::MethodInfo;
use self::attribute_info::AttributeInfo;

pub struct InstanceKlass {
    magic_number:u32,
    major_version: u16,
    minor_version: u16,
    constant_pool: ConstantPool,
    access_flag: u16,
    this_class_name: u16,
    super_class_name: u16,
    interface_length: u16,
    interfaces: Vec<InterfaceInfo>,
    field_length: u16,
    fields: Vec<FieldInfo>
    method_length: u16,
    methods: Vec<MethodInfo>,
    attribute_length: u16,
    attributes: Vec<AttributeInfo>,
}

