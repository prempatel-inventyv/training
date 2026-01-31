mod files;

fn main() {
    files::mutex::main();
    println!();
    files::rwlock::main();
}
