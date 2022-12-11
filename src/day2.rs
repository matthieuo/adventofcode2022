use std::fs;

pub fn day2(file_name: &str) -> (i32,i32) {
    let f_str = fs::read_to_string(file_name)
        .expect("Read error");

    let a1:i32 = f_str.lines().map(|x| config_to_score_part1(x)).sum();
    let a2:i32 = f_str.lines().map(|x| config_to_score_part2(x)).sum();

    return (a1,a2);
}


fn score_battle(c1 :char, c2 :char) -> i32 {

    let mut score = 0;
    
    if (c1 == 'A' && c2 == 'X') || (c1 == 'B' && c2 == 'Y') || (c1 == 'C' && c2 == 'Z') {
	score = 3;
    } else if c1 == 'A' && c2 == 'Y' {
	score = 6;
    } else if c1 == 'B' && c2 == 'Z' {
	score = 6;
    } else if c1 == 'C' && c2 == 'X' {
	score = 6;
    }

    if c2 == 'Y' {
	score += 2;
    } else if c2 == 'Z' {
	score += 3
    } else if c2 == 'X' {
	score += 1;
    }

    return score;
}
    
fn config_to_score_part1(conf: &str) -> i32 {
    let c1 = conf.chars().nth(0).unwrap();
    let c2 = conf.chars().nth(2).unwrap();

    return score_battle(c1, c2);
}


fn set_win(c: char) -> char {
    match c {
	'A' => 'Y',
	'B' => 'Z',
	'C' => 'X',
	_ => panic!("mmh"),
    }
}

fn set_loose(c: char) -> char {
    match c {
	'A' => 'Z',
	'B' => 'X',
	'C' => 'Y',
	_ => panic!("mmh"),
    }
}

fn set_eq(c: char) -> char {
    match c {
	'A' => 'X',
	'B' => 'Y',
	'C' => 'Z',
	_ => panic!("mmh"),
    }
}

fn config_to_score_part2(conf: &str) -> i32 {
    let c1 = conf.chars().nth(0).unwrap();
    let c2 = conf.chars().nth(2).unwrap();

    let new_c2;
    match c2 {
	'X' =>  new_c2 = set_loose(c1),
	'Y' => new_c2 = set_eq(c1),
	'Z' => new_c2 = set_win(c1),
	_ => panic!("mmh"),
    }
    return score_battle(c1, new_c2);
}

