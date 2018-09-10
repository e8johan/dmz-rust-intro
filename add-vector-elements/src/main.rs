fn add_elements(v: &Vec<i32>) -> i32 {
    return v[0] + v[1];
}

fn main() {
    let v = vec![0, 1, 2, 3];
    let answer = add_elements(&v);
    println!("{} + {} = {}", v[0], v[1], answer);
}
