use regex::Regex;

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new() -> Set {
        Set {
            red: 0,
            green: 0,
            blue: 0
        }
    }

    fn set_color(&mut self, color: &str, count: u32) {
        match color {
            "red" => self.red = count,
            "green" => self.green = count,
            "blue" => self.blue = count,
            _ => panic!("wrong color {}", color)
        }
    }
}

fn read_input() -> Vec<Game> {
    let mut games = vec![];
    let pattern = Regex::new(r"Game (\d+): (.+)").unwrap();

    for row in include_str!("input/day_02.txt").split("\n") {
        let captures = pattern.captures(row).unwrap();
        let game_id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let mut sets = vec![];
        for set_str in captures.get(2).unwrap().as_str().split("; ") {
            let mut set = Set::new();

            for color_str in set_str.split(", ") {
                let slice = color_str.split(" ").collect::<Vec<_>>();
                let [count, color] = slice.as_slice() else {
                    panic!("Color format")
                };
                let count = count.parse::<u32>().unwrap();
                set.set_color(color, count)
            }

            sets.push(set)
        }

        games.push(
            Game {
                id: game_id,
                sets
            }
        )
    }

    games
}

pub(crate) fn part_1() -> usize {
    let mut possible = 0;
    for game in read_input() {
        let combined_max = game.sets.into_iter().reduce(|a, b| Set {
            red: a.red.max(b.red),
            green: a.green.max(b.green),
            blue: a.blue.max(b.blue),
        }).unwrap();

        if combined_max.red <= 12 && combined_max.green <= 13 && combined_max.blue <= 14 {
            possible += game.id;
        }
    }
    possible
}
