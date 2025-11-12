use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Resource {
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListResource {
    pub uri: String,
}