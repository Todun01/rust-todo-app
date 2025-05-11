use std::env;
use rust_todo_app;
fn main() {
    let _args: Vec<String> = env::args().collect();
    rust_todo_app::run_app(&_args);
}

