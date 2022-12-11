use std::fs;
use std::collections::HashSet;


pub fn day3(file_name: &str) -> (i32,i32) {
    let f_str = fs::read_to_string(file_name)
        .expect("Read error");

    let a1:i32 = f_str.lines()
        .map(|l| split_str_two(l))
        .map(|v| find_common(&v.0, &v.1))
        .map(|v| v.iter()
	     .map(|&v1| to_int(v1))
	     .fold(0, |acc, x| acc + x as i32))
        .sum();

    let a2:i32 = f_str.lines().step_by(3)
	.zip(f_str.lines().skip(1).step_by(3))
	.zip(f_str.lines().skip(2).step_by(3))
	.map(|a| find_common_three(&String::from(a.0.0), &String::from(a.0.1),&String::from(a.1)))
	.map(|v| v.iter()
	     .map(|&v1| to_int(v1))
	     .fold(0, |acc, x| acc + x as i32))
        .sum();

    return (a1,a2);
}

fn split_str_two(st :&str) -> (String,String) {
    let str_len = st.chars().count();

    let st1 = st.chars().take(str_len / 2).collect::<String>();
    let st2 = st.chars().skip(str_len / 2).collect::<String>();

    return (st1,st2);
}

fn to_int(c: char) -> u8 {
    match c {
	'a'..='z' => 1 + c as u8 - 'a' as u8,
	'A'..='Z' => 27 + c as u8 - 'A' as u8,
	_ => panic!("mm"),
    }
}

fn find_common(str1 :&String, str2: &String) -> Vec<char>{
    let hs1 :HashSet<char> = str1.chars().collect();
    let hs2 :HashSet<char> = str2.chars().collect();

    let v :Vec<char> = hs1.intersection(&hs2).copied().collect();

    return v;
}
fn find_common_three(str1 :&String, str2: &String, str3: &String) -> Vec<char>{
    let hs1 :HashSet<char> = str1.chars().collect();
    let hs2 :HashSet<char> = str2.chars().collect();
    let hs3 :HashSet<char> = str3.chars().collect();

    let hs_tmp :HashSet<char> = hs1.intersection(&hs2).copied().collect();
    let v :Vec<char> = hs_tmp.intersection(&hs3).copied().collect();

    return v;
}
