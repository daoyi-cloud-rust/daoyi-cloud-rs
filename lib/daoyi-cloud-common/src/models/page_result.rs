use crate::models::page_param::{PAGE_SIZE_NONE, PageParam};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    list: Vec<T>,
    total: u64,
    page_no: u64,
    page_size: u64,
    total_page: u64,
}

impl<T> PageResult<T> {
    pub fn new(list: Vec<T>, total: u64, page_no: u64, page_size: u64) -> Self {
        let total_page = total / page_size + if total % page_size > 0 { 1 } else { 0 };
        PageResult {
            list,
            total,
            page_no,
            page_size,
            total_page,
        }
    }

    pub fn from_pagination(pagination: PageParam, total: u64, list: Vec<T>) -> Self {
        let mut page_size = pagination.page_size as u64;
        if pagination.page_size == PAGE_SIZE_NONE {
            page_size = list.len() as u64;
        }
        PageResult::new(list, total, pagination.page_no, page_size)
    }
}
