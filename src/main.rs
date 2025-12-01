use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut dial_pos: i16 = 50;
    let path = "./input.txt";
    let mut password: i32 = 0;
    //println!("Position is now: {}", turn(&line, dial_pos));

    //proper

    println!("The dial starts by pointing at {}.", dial_pos);
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            dial_pos = turn(&line, dial_pos);
            if dial_pos == 0 {
                password += 1;
            };
        }
    }
    println!("Password seems to be {}", password);
}

fn turn(motion: &str, position: i16) -> i16 {
    let mut turns: i32 = motion[1..].parse::<i32>().expect(
        "Input should follow the format of L or R as a first character, following by numbers",
    );
    let pos: i32 = position as i32;

    if motion.starts_with("L") {
        turns = -turns;
    };
    let mut result = ((pos + turns) % 100) as i16;
    while result < 0 {
        result += 100;
    }
    println!("The dial is rotated {} to point at {}.", motion, result);
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
