pub mod collections;
pub mod direction;
pub mod point;

pub trait Solution {
    fn name(&self) -> String;
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

pub fn load(filename: &str) -> String {
    let file = format!("data/{}.txt", filename);
    read_to_string(&file).unwrap_or_else(|_| panic!("Error reading file {}", file))
}

pub fn input(input: &str) -> Option<String> {
    print!("{}", input);

    let mut line = String::new();
    stdout().flush().ok()?;
    stdin().read_line(&mut line).ok()?;
    while line.ends_with('\n') || line.ends_with('\r') {
        line.pop();
    }

    Some(line)
}
