use std::error::Error;
use std::fmt;
use std::fmt::{Formatter, write};

#[derive(Debug)]
pub enum CustomError{
    NoPageFound,
    SidebarGenerateError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::NoPageFound => write!(f, "No pages found to generate sidebar items"),
            CustomError::SidebarGenerateError(page) => write!(f, "Failed to load sidebar item for page {}", page)

        }
    }
}

impl Error for CustomError{}