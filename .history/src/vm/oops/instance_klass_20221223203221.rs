pub struct InstanceKlass {
    pub magic_number:u32,
    pub version_number: VersionNumber,

}

struct VersionNumber {
    pub major_version: u16,
    pub minor_version: u16,
}