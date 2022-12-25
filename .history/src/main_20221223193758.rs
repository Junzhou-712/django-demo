extern crate jvm;

use jvm::classfile::boot_class_loader::BootClassLoader;


fn main() {
    start_jvm();
}

fn start_jvm() {
    BootClassLoader::load_main_klass("src.data.HelloWorld");
    
}