pub trait NextUrl {
    type Output;
    fn next_url(&self) -> Option<Self::Output>;
    fn has_next(&self) -> bool;
}

pub trait Pagible {
    type Output;
    fn prev(&self) -> Option<Self::Output>;
    fn next(&self) -> Option<Self::Output>;
}