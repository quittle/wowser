use std::error::Error;
use wowser_macros::DisplayFromDebug;

#[derive(Debug, DisplayFromDebug)]
pub struct ExecutionError(pub String);

impl Error for ExecutionError {}
