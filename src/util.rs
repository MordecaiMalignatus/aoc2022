use anyhow::Result;

const ROOT_PATH: &str = "./input/";

pub fn read_input(day: i32) -> Result<String> {
    let conv = day.to_string();
    Ok(std::fs::read_to_string(format!("{}/day_{}.txt", ROOT_PATH, conv))?)
}
