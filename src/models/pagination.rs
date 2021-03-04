use serde::Serialize;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Pagination {
  pub page: i32,
  pub items_per_page: i32,
  pub total_pages: i32,
  pub total_count: i64,
}
