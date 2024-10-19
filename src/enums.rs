#[derive(Debug)]

pub enum ParseError {
    InvalidHtml(String), 
    MissingField(String), 
    Other(String),
}