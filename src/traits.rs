use crate::client::api::ApiResult;

pub trait NextUrl {
    type Output;
    fn next_url(&self) -> Option<ApiResult<Self::Output>>;
    fn has_next(&self) -> bool;
}

pub trait Pagible {
    type Output;
    fn prev(&self) -> Option<Self::Output>;
    fn next(&self) -> Option<Self::Output>;
}