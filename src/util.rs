use anyhow::Result;

const ROOT_PATH: &str = "./input/";

/// Read the input file associated with a day.
pub fn read_input(day: i32) -> Result<String> {
    let conv = day.to_string();
    Ok(std::fs::read_to_string(format!(
        "{ROOT_PATH}/day_{conv}.txt"
    ))?)
}

/// Split text in to paragraphs
pub fn paragraphs(input: &'_ str) -> std::str::Split<'_, &'_ str> {
    input.split("\n\n")
}

/// Tap allows stuff that would require statements to be closures instead.
pub trait Tap {
    fn tap(self, fun: fn(&Self) -> ()) -> Self;
    fn tap_mut(self, fun: fn(&mut Self) -> ()) -> Self;
}
impl<T> Tap for Vec<T> {
    fn tap(self, fun: fn(&Self) -> ()) -> Self {
        fun(&self);
        self
    }

    fn tap_mut(mut self, fun: fn(&mut Self) -> ()) -> Self {
        fun(&mut self);
        self
    }
}
