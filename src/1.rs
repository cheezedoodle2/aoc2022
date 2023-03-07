use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("1.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("Read {} input bytes.\n", s.len()),
    }

    let mut maxcount: usize = 0;
    let mut count: usize = 0;

    s.split('\n').for_each(|line| {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if count > maxcount {
                maxcount = count;
            }
            count = 0;
        }
        else {
            match trimmed.parse::<usize>() {
                Err(_) => panic!("Error parsing number."),
                Ok(n) => { count = count + n },
            }
        }
    });

    println!("maxcount: {}", maxcount);
}