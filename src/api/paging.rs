//! cnMaestro's API is very web-oriented and pages
//! responses. These are helpers to make that a little
//! easier to work with.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Wrapper<T> {
    pub paging: Paging,
    pub data: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct Paging {
    pub limit: u32,
    pub total: u32,
}
