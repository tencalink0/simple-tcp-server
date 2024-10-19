pub fn get_user_input() -> String {
    use std::io::{stdin, stdout, Write};
    let mut input = String::new();
    let _ = stdout().flush();

    stdin()
        .read_line(&mut input)
        .expect("An error occured");

    //clean string
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    input
}