use http_server::cl_args;

fn main() {
    let request_path = cl_args::get_args();

    match request_path {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
