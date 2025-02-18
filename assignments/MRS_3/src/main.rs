#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    for i in low..=high { // inclusive loop
        *total += i;
    }   
}

fn main() {
    let mut total = 0; // init
    sum(&mut total, 0, 100); // call fun
    println!("{}", total)
}