mod day1;
mod day2;

fn main() {
    let (d1_max, d1_top3) = day1::day1("/home/matthieu/dev/adventofcode2022/adv2022/input_day1");
    let (d2_p1, d2_p2) = day2::day2("/home/matthieu/dev/adventofcode2022/adv2022/input_day2");
    println!("Day 1: {d1_max} {d1_top3}");
    println!("Day 2: {d2_p1} {d2_p2}");
    

    
}
