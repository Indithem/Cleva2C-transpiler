#[cfg_attr(test, derive(Clone))]
pub enum Literals{
    Number(f64),
    Boolean(bool),
    String(String),
}