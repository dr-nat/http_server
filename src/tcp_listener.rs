use tokio::net::TcpListener;
use crate::request::handle_request;
 
type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>; 

pub async fn create_listener() -> GenericResult<()>{

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Waiting for request: -------");
    
    loop {

        let (socket, _) = listener.accept().await?; // so the accept method returns the socket which is the stream and connection address, but we are only using the socket(connection)

        tokio::spawn(async move {
            match handle_request(socket).await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to read from socket {}", e);
                    return;
                }
            }
        });

        return Ok(());

    }
}

