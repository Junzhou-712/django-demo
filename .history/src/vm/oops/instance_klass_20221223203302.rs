use self::constant_pool::ConstantPool;

pub struct InstanceKlass {
    pub magic_number:u32,
    pub version_number: VersionNumber,
    pub constant_pool: ConstantPool,

}

struct VersionNumber {
    pub major_version: u16,
    pub minor_version: u16,
}