use vec_map::VecMap;
use self::constant_pool::ConstantPool;

pub struct InstanceKlass {
    pub magic_number:u32,
    pub version_number: VersionInfo,
    pub constant_pool: ConstantPool,
    pub access_flag: u16,
    pub this_class_name: u16,
    pub super_class_name: u16,
    pub interface_length: u16,
    pub interfaces: Vec<InterfaceInfo>,
    pub field_length: u16,
    pub fields: Vec<FieldInfo>
    pub method_length: u16,
    pub methods: Vec<MethodInfo>,
    pub attribute_length: u16,
    pub attributes: Vec<AttributeInfo>,
}

struct VersionInfo {
    pub major_version: u16,
    pub minor_version: u16,
}