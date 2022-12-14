use anyhow::Result;

const ROOT_PATH: &str = "./input/";

pub fn read_input(day: i32) -> Result<String> {
    let conv = day.to_string();
    Ok(std::fs::read_to_string(format!("{ROOT_PATH}/day_{conv}.txt"))?)
}

pub trait Tap {
    fn tap(self, fun: fn(Self)->Self) -> Self;
}
impl<T> Tap for Vec<T> {
    fn tap(self, fun: fn(Self) -> Self) -> Self {
        fun(self)
    }
}
