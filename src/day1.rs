use std::fs;
use std::str::Split;

pub fn day1(){
    crate::advent::day_intro(1);
    const FILE_PATH: &str = "/usr/src/myapp/src/day1.txt";

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let values: Split<char> = contents.split('\n');
    let mut snacks_total: Vec<i32> = aggregate_sums(values);

    snacks_total.sort();
    println!("Most caloric elf : {}",snacks_total.iter().max().unwrap());

    let nb_backups=3;
    let backups = &snacks_total[snacks_total.len()-nb_backups..];
    println!("Sum of 3 most caloric elves : {}",backups.iter().sum::<i32>());
}

fn aggregate_sums(iter: Split<char>) -> Vec<i32>
{

    let mut acc:Vec<i32> = Vec::new();
    let mut sub_total:i32 = 0;
    for value in iter {
        if value=="" {
            acc.push(sub_total);
            sub_total=0
        }else {
            sub_total+= value.parse::<i32>().unwrap();
        }
    }
    acc
}
