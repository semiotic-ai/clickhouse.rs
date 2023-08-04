use crate::Row;

pub trait Schema {
    fn get_columns(&self) -> Vec<&str>;
}

impl<T> Schema for T
where
    T: Row,
{
    fn get_columns(&self) -> Vec<&str> {
        T::COLUMN_NAMES.iter().map(|col| *col).collect()
    }
}

struct A {
    columns: Vec<String>,
}

impl A {
    fn get_columns(&self) -> Vec<&str> {
        self.columns
            .iter()
            .map(|col| col.as_str())
            .collect::<Vec<_>>()
            // .as_slice()
    }
}
