use crate::Row;

pub trait Schema {
    fn get_columns(&self) -> &[&'static str];
}

impl<T> Schema for T
where
    T: Row,
{
    fn get_columns(&self) -> &[&'static str] {
        T::COLUMN_NAMES
    }
}
