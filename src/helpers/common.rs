use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: usize,
    pub size: usize,
    pub has_next_page: bool,
}

impl<T> PaginatedResponse<T> {
    pub fn from_rows(mut raw: Vec<T>, page: i64, size: i64) -> PaginatedResponse<T> {
        let len = raw.len();
        let has_next_page = len > size as usize;

        PaginatedResponse {
            data: if has_next_page {
                raw.split_off(size as usize)
            } else {
                raw
            },
            page: page as usize,
            size: if has_next_page { size as usize } else { len },
            has_next_page,
        }
    }
}
