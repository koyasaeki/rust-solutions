fn main() {
    if let Err(e) = catr::run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
