fn main() {
    let input = read_input();

    let vec: Vec<i32> = input.chars().map(|a| a.to_string().parse::<i32>().unwrap()).collect();
    let next_sum = vec.iter().enumerate().filter(|a| matches_next(&vec, a.0)).fold(0, |sum, x| sum+x.1);
    let around_sum = vec.iter().enumerate().filter(|a| matches_around(&vec, a.0)).fold(0, |sum, x| sum+x.1);
    
    println!("Sum when using matches next {}", next_sum);
    println!("Sum when using matches halfway around {}", around_sum);

}

fn matches_next(vec: &Vec<i32>, n: usize) -> bool{
    if n >= vec.len() {
        false
    } else if n == vec.len() - 1 {
        vec[n] == vec[0]
    } else {
        vec[n] == vec[n+1]
    }
}

fn matches_around(vec: &Vec<i32>, n: usize) -> bool {
    vec[n] == vec[(n + vec.len()/2 as usize) % vec.len()]
}

fn read_input() -> String {
    use std::io;
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}