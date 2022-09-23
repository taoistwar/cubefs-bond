use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct BodyNotExists {}

impl Error for BodyNotExists {}

impl Display for BodyNotExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
