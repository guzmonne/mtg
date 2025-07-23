use crate::prelude::*;

pub struct Params {
    pub log_path: Option<String>,
    pub filter: Option<Vec<String>>,
    pub format: String,
}

pub async fn run(params: Params) -> Result<()> {
    todo!()
}
