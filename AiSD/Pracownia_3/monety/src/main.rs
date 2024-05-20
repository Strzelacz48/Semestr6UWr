use std::{io, vec};

//f masa pudelka ktore chcemy osiagnac, c liczba monet, coins wektor par monet (min, max)
fn find_sol(f : i32, c : i8, coins : Vec<(i32, i32)>) -> Vec<(i8, i8, i32, i32)>{
    
    let mut backpacks : Vec<(i8, i8, i32, i32)> = Vec::new();
    backpacks.push((0, 0, 0, 0));

    for i in 1..f + 1{
    backpacks.push((-1, -1, -1, -1));

        for j  in 0..c{
            let iu = i as usize;
            let ju = j as usize;
            if i < coins[ju].1{
                continue;
            }
            let prev_backpack = backpacks[(i - coins[ju].1) as usize];
            if prev_backpack.0 == -1{
                continue;}

            if backpacks[iu].2 > prev_backpack.2 + coins[ju].0
            || backpacks[iu].2 == -1{
                backpacks[iu].0 = j;//min_idx = j;
                backpacks[iu].2 = prev_backpack.2 + coins[ju].0; //min_val = prev_backpack.2 + coins[ju].0;
            }
            if backpacks[iu].3 < prev_backpack.3 + coins[ju].0
            || backpacks[iu].3 == -1{
                backpacks[iu].1 = j;//max_idx = j;
                backpacks[iu].3 = prev_backpack.3 + coins[ju].0; //max_val = prev_backpack.3 + coins[j as usize].0;
            }
        }
    }
    backpacks
}

fn recreate_solution(coins : Vec<(i32, i32)>, backpacks: Vec<(i8, i8, i32, i32)>){
    if backpacks[backpacks.len() - 1].0 == -1{
        println!("NIE");
        return;
    }
    println!("TAK");// wez ostatni element z backpacks i przejdz sie po tych backpackach od konca do poczatku odejmujac wartosci z coins o indeksach z backpacks
    println!("{:?}", backpacks[backpacks.len() - 1].2);
    //tutaj wypisz monety ktore sa w plecaku min
    let mut how_many = vec![0; coins.len()];
    let mut i = backpacks.len() - 1;
    while i > 0{
        let min_idx = backpacks[i].0;
        how_many[min_idx as usize] += 1;
        i -= coins[min_idx as usize].1 as usize;
    }
    for i in 0..coins.len(){
        print!("{} ", how_many[i]);
    }
    println!("\n{:?}", backpacks[backpacks.len() - 1].3);
    how_many = vec![0; coins.len()];
    i = backpacks.len() - 1;
    while i > 0{
        let max_idx = backpacks[i].1;
        how_many[max_idx as usize] += 1;
        i -= coins[max_idx as usize].1 as usize;
    }
    for i in 0..coins.len(){
        print!("{} ", how_many[i]);
    }
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
    //println!("{:?}", coins);
    let result = find_sol(f, c, coins.clone());
    //println!("{:?}", result);
    recreate_solution(coins, result);
}