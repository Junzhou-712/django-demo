use vm::classfile::boot_class_loader::BootClassLoader;


fn main() {
    start_jvm();
}

fn start_jvm() {
    let main_klass = BootClassLoader::load_main_klass("src.data.HelloWorld");
    
}