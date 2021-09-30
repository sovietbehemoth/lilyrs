



fn main() {
    let sample = "
    000000000000000100000000000000000000000000000100000000000000000100000000000000000000000000010000000000000
    000000000000000000010000000000000000000000000000000000000100000000000000000000000001000000000000000000010
    ";

    let mut repl = String::new();

    for i in 0..sample.len() {
        let current_char:char = sample.chars().nth(i).unwrap();
        if current_char == '1' {
            repl.push_str("ONE");
        } else {
            repl.push(current_char);
        }
    }

    let nums:[f64;16] = [
        1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 
        64.0, 81.0, 100.0, 121.0, 144.0, 
        169.0, 196.0, 225.0, 256.0];

    for _ in 0..200 {
        for i in 0..nums.len() {
            let _ = nums[i as usize].sqrt();
        }
    }

    println!("Rust: Completed all iterations.");
}