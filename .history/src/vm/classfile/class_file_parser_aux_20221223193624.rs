
use self::class_loader::ClassLoader;
use oops::attribute_info::{
    AttributeInfo, ExceptionTableEntry, LineNumberTableEntry, LocalVariableTableEntry,
};
use oops::constant_info::ConstantInfo;
use oops::constant_pool::ConstantPool;
use oops::member_info::MemberInfo;

#[derive(Debug)]
pub struct VersionInfo {
    major_version: u16,
    minor_version: u16,
}

pub 

