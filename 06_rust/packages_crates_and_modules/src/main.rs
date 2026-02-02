mod files;

fn main() {
    println!();
    println!("Loop Task");
    println!();
    files::_01_loop::loop_task();
    println!();
    println!("Structure Task");
    println!();
    files::_02_struct::struct_task();
    println!();
    println!("Serde Json Task");
    println!();
    files::_03_serde_json::serde_json_task();
    println!();
    println!("Mutex Task");
    println!();
    files::_04_mutex::mutex_task();
    println!();
    println!("RwLock Task");
    println!();
    files::_05_rwlock::rwlock_task();
}
