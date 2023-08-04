use crate::Row;

pub trait Schema {
    fn get_columns<'a>(&self) -> &[&'a str];
}

impl<T> Schema for T
where
    T: Row,
{
    fn get_columns<'a>(&self) -> &[&'a str] {
        T::COLUMN_NAMES
    }
}
