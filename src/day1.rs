use std::fs;

pub fn day1(file_name: &str) -> (i32,i32) {

    let f_str = fs::read_to_string(file_name)
        .expect("Read error");
    let mut runing_int = 0;
    let mut v: Vec<i32> = Vec::new();
    
    for line in f_str.lines() {
	if line.is_empty() {
	    v.push(runing_int);
	    runing_int = 0;
	}
	else {
	    runing_int += line.parse::<i32>().unwrap();
	}
    }

    v.sort();
    v.reverse();
    
    let top3:i32 = v[0..3].iter().sum();

    return (v[0],top3);
}


