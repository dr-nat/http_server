use std::collections::HashMap;

pub enum Methods {
    Get,
    Patch,
    Post,
    Delete,
}


pub struct Request {
    method: Methods,
    host: String, 
    path: String,
    header: HashMap<String, String>,
    body: Option<Vec<u8>>,
}
