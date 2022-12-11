mod day1;
mod day2;
mod day3;

fn main() {
    let (d1_p1, d1_p2) = day1::day1("/home/matthieu/dev/adventofcode2022/adv2022/input_day1");
    let (d2_p1, d2_p2) = day2::day2("/home/matthieu/dev/adventofcode2022/adv2022/input_day2");
    let (d3_p1, d3_p2) = day3::day3("/home/matthieu/dev/adventofcode2022/adv2022/input_day3");
    
    println!("Day 1: {d1_p1} {d1_p2}");
    println!("Day 2: {d2_p1} {d2_p2}");
    println!("Day 3: {d3_p1} {d3_p2}");
}
