pub trait Schema {
    fn get_columns(&self) -> Vec<&str>;
}
