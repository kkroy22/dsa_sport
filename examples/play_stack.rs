use dsa_sport::datastruct::vec_struct::Vector;

fn main() {
    let line_1 = "()()(()";
    let line_2 = "(()())";
    let mut stack_1 = Vector::new();

    for i in 0..line_1.len() {
        stack_1.push_back(line_1.chars().nth(i).unwrap());
    }
    println!("{}", stack_1);

}
