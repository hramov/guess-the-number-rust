pub fn get_user_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_user_number() -> i32 {
    let input = get_user_string();
    input.parse::<i32>().expect("Please type a number!")
}