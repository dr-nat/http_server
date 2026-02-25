use std::error::Error;


#[derive(PartialEq)]
#[derive(Debug)]
pub enum Methods {
    GET,
    PATCH,
    POST,
    DELETE,
    UNKNOWN
}

// so we are creating an implementation block for the to string trait 
// we want the enum methods to implement the tostring method found under 
// to string trait, so we create our own method in an impl block.
impl ToString for Methods {
    fn to_string(&self) -> String {
       match self {
            Methods::GET => "GET".to_string(),
            Methods::PATCH => "PATCH".to_string(),
            Methods::POST => "POST".to_string(),
            Methods::DELETE => "DELETE".to_string(),
            Methods::UNKNOWN => "UNKNOWN".to_string(),
       } 
    } 
}

pub struct Request {
    method: Methods,
    host: String, 
    path: String,
}


impl Request {
    pub fn new(request: &str) -> Result<Self, Box<dyn Error>>{

        //METHODS
        let mut lines = request.lines(); 

        let first_line = &lines.next().ok_or("first line not found")?;

        let mut words = first_line.split_whitespace();

        let first_word = words 
            .next()
            .ok_or("First word not found")?;


        let methods = match first_word {  
            "GET" => Methods::GET,

            "PATCH" => Methods::PATCH ,
            
            "POST" => Methods::POST,

            "DELETE" => Methods::DELETE, 

            _ => Methods::UNKNOWN,
        };
        
        //PATH
        let request_path = words.next().ok_or("Path not found")?; 
        
        //HOST
        let host = request.lines()
            .find(|line| line.starts_with("Host: "))
            .and_then(|line| line.strip_prefix("Host: "))
            .ok_or("Host header not found")?
            .to_string();
    

        Ok(Request{
            method: methods,
            host: host,
            path: request_path.to_string(),
        })
    }


    pub fn get_method(&self) -> Methods  {
        self.method
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_host(&self) -> String {
        self.host.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::models;

    const _DUMMY_REQUEST: &str = "GET /index.html HTTP/1.1\r\n
Host: localhost:8080\r\n
User-Agent: Mozilla/5.0\r\n
Accept: */*\r\n\r\n"; 


    #[test]
    fn test_method(){
        let request: models::Request = Request{
            method: Methods::GET,
            host: "example.com".to_string(),
            path: "/index.html".to_string(),
        };


        assert_eq!(request.get_method(), &Methods::GET) 
    }

    #[test]
    fn test_path() {
        let request: models::Request = Request{
                method: Methods::GET,
            host: "example.com".to_string(),
            path: "/index.html".to_string(),
        };


        assert_eq!(request.get_path(), "/index.html")
    }


    #[test]
    fn get_host() {
        let request: models::Request = Request{
            method: Methods::GET,
            host: "localhost.8080".to_string(),
            path: "/index.html".to_string(),
        };


        assert_eq!(request.get_host(), "localhost:8080")
    }
}       
