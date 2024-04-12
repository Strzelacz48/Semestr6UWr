use std::io;

//f masa pudelka ktore chcemy osiagnac, c liczba monet, coins wektor par monet (min, max)
fn find_sol(f : i32, c : i8, coins : Vec<(i32, i32)>) -> Vec<(i8, i8, i32, i32)>{
    
    let mut F : Vec<(i8, i8, i32, i32)> = Vec::new();
    F.push((0, 0, 0, 0));

    for i in 1..f + 1{
    F.push((-1, -1, -1, -1));

        for j  in 0..c{
            let iu = i as usize;
            let ju = j as usize;
            if i < coins[ju].1{
                continue;
            }
            let prev_backpack = F[(i - coins[ju].1) as usize];
            if prev_backpack.0 == -1{
                continue;}

            if F[iu].2 > prev_backpack.2 + coins[ju].0
            || F[iu].2 == -1{
                F[iu].0 = j;//min_idx = j;
                F[iu].2 = prev_backpack.2 + coins[ju].0; //min_val = prev_backpack.2 + coins[ju].0;
            }
            if F[iu].3 < prev_backpack.3 + coins[ju].0
            || F[iu].3 == -1{
                F[iu].1 = j;//max_idx = j;
                F[iu].3 = prev_backpack.3 + coins[ju].0; //max_val = prev_backpack.3 + coins[j as usize].0;
            }
        }
        //F.push((min_idx, max_idx, min_val, max_val));
    }
    //let mut result: Vec<(i32, i32)> = Vec::new();
    F
    }


fn solution(warunek : bool) -> String{
    let pmin = 0;
    if warunek {
        return "TAK".to_string();
    }
    "NIE".to_string()
}

fn main() {
    //scan input data
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let f: i32 = input.trim().parse().unwrap();
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let c: i8 = input.trim().parse().unwrap();
    let mut coins: Vec<(i32, i32)> = Vec::new();
    for _ in 0..c{
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let pair: (i32, i32) = (input.split_whitespace().nth(0).unwrap().parse().unwrap(), input.split_whitespace().nth(1).unwrap().parse().unwrap());
        coins.push(pair);
    }
    println!("{:?}", coins);
    let result = find_sol(f, c, coins);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn a_test(){
        let f = 100;
        let c: i8 = 2;
        let mut coins: Vec<(i32, i32)> = vec![(1,1),(30,50)];
        assert_eq!(solution(f > 5), "TAK".to_string());
    }
    #[test]
    fn b_test(){
        let f = 10;
        let c: i8 = 3;
        let mut coins: Vec<(i32, i32)> = vec![(1, 1), (2, 4), (4, 16)];
        assert_eq!(solution(f > 5), "TAK".to_string());
    }
    #[test]
    fn c_test(){
        let f = 5;
        let c: i8 = 3;
        let mut coins: Vec<(i32, i32)> = vec![(1, 2), (1, 4), (2, 4)];
        assert_eq!(solution(f > 5), "NIE".to_string());
    }
}