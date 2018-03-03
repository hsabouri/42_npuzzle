pub fn exit(msg: &str) {
    println!("Error: {}", msg);
    ::std::process::exit(1);
}
