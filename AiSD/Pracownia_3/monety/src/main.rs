use std::io::{empty, Empty};
use std::io;

fn pair<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

fn solution(warunek : bool) -> String{
    let pmin = 0;
    if warunek {
        return "TAK".to_string();
    }
    "NIE".to_string()
}

fn main() {
    /*
    let a = (1, 2);
    let b= false;
    println!("{:?}", a);
    println!("{:?}", a.0);
    println!("Hello, world!");
    if !b{
        println!("NIE");
        //return;
    }
    println!("TAK"); */
    //scan input data
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let f: i32 = input.trim().parse().unwrap();
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let c: i8 = input.trim().parse().unwrap();
    let mut pairs: Vec<(i32, i32)> = Vec::new();
    for _ in 0..c{
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let pair: (i32, i32) = (input.split_whitespace().nth(0).unwrap().parse().unwrap(), input.split_whitespace().nth(1).unwrap().parse().unwrap());
        pairs.push(pair);
    }
    println!("{:?}", pairs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        assert_eq!(pair(1, 2), (1, 2));
    }
    #[test]
    fn a_test(){
        let f = 100;
        let c: i8 = 2;
        assert_eq!(solution(f > 5), "TAK".to_string());
    }
    #[test]
    fn b_test(){
        let f = 10;
        let c: i8 = 3;
        assert_eq!(solution(f > 5), "TAK".to_string());
    }
    #[test]
    fn c_test(){
        let f = 5;
        let c: i8 = 3;
        assert_eq!(solution(f > 5), "NIE".to_string());
    }
}