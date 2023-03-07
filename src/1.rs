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
    let mut top: Vec<usize> = vec![0, 0, 0];

    s.split('\n').for_each(|line| {
        let trimmed = line.trim();

        // If it's a blank line, see if we have a new high total calorie count.
        if trimmed.is_empty() {
            if count > maxcount {
                maxcount = count;
            }
            if count > top[0] {
                top[0] = count;
                top.sort();
            }
            count = 0;
        }
        // Otherwise, update the total calories.
        else {
            match trimmed.parse::<usize>() {
                Err(_) => panic!("Error parsing number."),
                Ok(n) => { count = count + n },
            }
        }
    });

    println!("maxcount: {}", maxcount);
    println!("top calories: ");
    let mut top_total: usize = 0;
    top.iter().for_each(|n| {
        top_total += n;
        println!("  {}", n);
    });
    println!("total calories for top {}: {}", top.len(), top_total);
}