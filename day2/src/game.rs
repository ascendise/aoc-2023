use regex::{Regex, Captures};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    sets: Vec<CubeSet>
}

impl Game {
    pub fn from(line: &str) -> Game {
        let game_regex = Regex::new(r"Game (?P<id>\d*): (?P<sets>.*)").unwrap();
        let game_capture = game_regex.captures(line).unwrap();
        let set_line = &game_capture["sets"];
        let sets = set_line.split(";")
            .map(|set| CubeSet::from(set))
            .collect();
        Game {
            id: game_capture["id"].parse().unwrap(),
            sets
        }
    }

    pub fn is_content(self: &Game, content: &CubeSet) -> bool {
        for set in &self.sets {
            if set.r > content.r || set.b > content.b || set.g > content.g {
                return false;
            }
        }
        true
    }

    pub fn get_minimum_content(self: &Game) -> CubeSet {
        let mut content: (u32, u32, u32) = (0, 0, 0);
        for set in &self.sets {
            if set.r > content.0 {
                content.0 = set.r;
            }
            if set.g > content.1 {
                content.1 = set.g;
            }
            if set.b > content.2 {
                content.2 = set.b;
            }
        }
        CubeSet::new(content.0, content.1, content.2)
    }
}

#[derive(Debug, PartialEq)]
pub struct CubeSet {
    r: u32,
    g: u32,
    b: u32
}

impl CubeSet {
    pub fn new(r: u32, g: u32, b: u32) -> CubeSet {
        CubeSet {
            r, g, b
        }
    }

    fn from(line: &str) -> CubeSet {
        let red_regex = Regex::new(r"(?P<r>\d*) red").unwrap();
        let green_regex = Regex::new(r"(?P<g>\d*) green").unwrap();
        let blue_regex = Regex::new(r"(?P<b>\d*) blue").unwrap();
        CubeSet {
            r: Self::get_count("r", &(red_regex.captures(line))),
            g: Self::get_count("g", &(green_regex.captures(line))),
            b: Self::get_count("b", &(blue_regex.captures(line)))
        }
    }

    fn get_count(color_name: &str, captures: &Option<Captures>) -> u32 {
        match captures {
            Some(c) => c[color_name].parse().unwrap(),
            None => 0
        }
    }

    pub fn power(self: &CubeSet) -> u32 {
        self.r * self.g * self.b
    }
}