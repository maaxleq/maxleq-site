use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[derive(Default)]
pub struct Pagination {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}
