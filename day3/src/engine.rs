#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
struct Engine {
    matrix: Vec<Vec<char>>
}

impl Engine {

    pub fn from(lines: Vec<&str>) -> Engine {
        let matrix = lines
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Engine {
            matrix
        }
    }

}