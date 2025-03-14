use crate::errors::Error;

pub type Result<T> = anyhow::Result<T, Error>;
