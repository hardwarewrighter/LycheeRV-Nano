fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => {
            println!("Hello, Lychee!");
            println!("Rust CLI running on {} Linux", std::env::consts::ARCH);
        }
        [flag] if flag == "--help" || flag == "-h" => {
            println!("Usage: hello-lychee [--help | --version]");
            println!("A Rust hello-world CLI for the Sipeed LicheeRV Nano.");
        }
        [flag] if flag == "--version" || flag == "-V" => {
            println!("hello-lychee {}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("Usage: hello-lychee [--help | --version]");
            std::process::exit(2);
        }
    }
}
