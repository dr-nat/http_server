use http_server::cl_args;
use crate::tcp_listener::create_listener;
use http_server::tcp_listener;

#[tokio::main]
async fn main() {
    let request_path = cl_args::get_args();

    match request_path {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    let request_stream = create_listener();

    match request_stream.await {
        Ok(_) => {},
        Err(e) => {
            eprint!("Error: {}", e);
            std::process::exit(1);
        }
    } 


}
