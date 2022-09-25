use serde::Serialize;

#[derive(Debug)]
pub struct Res<T>
where
    T: Serialize + Default,
{
    code: u32,
    msg: String,
    data: Option<T>,
}

impl<T> Default for Res<T>
where
    T: Serialize + Default,
{
    fn default() -> Self {
        Self {
            code: 0,
            msg: "OK".to_owned(),
            data: Default::default(),
        }
    }
}
