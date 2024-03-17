use std::io;

fn tree_pre_procces(tree_vec: &Vec<(i32,i32)>, in_out_time:&mut Vec<(i32,i32)>, child_idx: Vec<i32>, root: i32, time: i32)->i32{
    //println!("DFS root- {}",root);
    //println!("in_time - {}", time);
    //println!("in_out_time");
    //println!("{:?}", in_out_time);
    if child_idx[root as usize - 1] == -1 {
        in_out_time[root as usize - 1] = (time, time);
        return time;
    }
    let mut i = child_idx[root as usize - 1] as usize;//i.unwrap();
    //println!("i - {}",i);
    let in_time = time;
    let mut out_time = time;
    
    while i < tree_vec.len() && tree_vec[i].0 == root{
        //println!("DFS child- {}",tree_vec[i].1);
        out_time = tree_pre_procces(tree_vec, in_out_time, child_idx.clone(), tree_vec[i].1, out_time + 1);
        //println!("out_time - {}", out_time);
        i+=1;
    }
    in_out_time[root as usize - 1] = (in_time, out_time);
    
    //println!("out_time - {}", time);
    out_time
}

fn is_ancestor(in_out_time: Vec<(i32, i32)>, query: (i32, i32)) -> bool{
    if in_out_time[query.0 as usize - 1].0 <= in_out_time[query.1 as usize - 1].0 && in_out_time[query.0 as usize - 1].1 >= in_out_time[query.1 as usize - 1].1{
        return true
    }
    false
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = input[0];
    let m = input[1];
    let mut tree_vec: Vec<(i32,i32)> = Vec::new();
    //Tree input
    for i in 1..n{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        tree_vec.push((input.trim().parse().unwrap(), i+1));
    }
    //println!("{:?}", tree_vec);
    tree_vec.sort_by(|a, b| a.0.cmp(&b.0));
    //fill child_idx array
    let mut child_idx = vec![-1; n as usize];
    let mut curr_father = 1;
    child_idx[0] = 0;
    for i in 0..n-1{
        if tree_vec[i as usize].0 != curr_father{
            curr_father = tree_vec[i as usize].0;
            child_idx[curr_father as usize - 1] = i;
        }
    }
    //println!("child_idx : {:?}", child_idx);
    
    let mut in_out_time: Vec<(i32,i32)> = vec![(0,0);n as usize];
    tree_pre_procces(&tree_vec, &mut in_out_time, child_idx.clone(), 1, 1);
    //println!("in_out_time {:?}", in_out_time);
    //Query
    let mut query: Vec<(i32,i32)> = vec![(0,0);m as usize];
    for i in 0..m{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        query[i as usize] = (input.split_whitespace().nth(0).unwrap().parse().unwrap(), input.split_whitespace().nth(1).unwrap().parse().unwrap());
        if is_ancestor(in_out_time.clone(), query[i as usize]){
            println!("TAK");
        }else{
            println!("NIE");
        }
    }
    //println!("Query {:?}", query);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a(){
        let tree_input = vec![(1,2),(1,3),(3,4),(3,5)];
        let query = vec![(1,2),(2,1),(1,4),(2,5)];
        let n = 5;
        let mut in_out_time: Vec<(i32,i32)> = vec![(0,0);n as usize];
        //fill child_idx array
        let mut child_idx = vec![-1; n as usize];
        let mut curr_father = 1;
        child_idx[0] = 0;
        for i in 0..n-1{
            if tree_input[i as usize].0 != curr_father{
                curr_father = tree_input[i as usize].0;
                child_idx[curr_father as usize - 1] = i;
            }
        }
        println!("child_idx : {:?}", child_idx);

        tree_pre_procces(&tree_input, &mut in_out_time, child_idx.clone(),1, 1);
        assert_eq!(in_out_time.clone(), vec![(1,5), (2,2), (3,5), (4,4), (5,5)]);
        assert_eq!(is_ancestor(in_out_time.clone(), query[0]), true);
        assert_eq!(is_ancestor(in_out_time.clone(), query[1]), false);
        assert_eq!(is_ancestor(in_out_time.clone(), query[2]), true);
        assert_eq!(is_ancestor(in_out_time.clone(), query[3]), false);
    }
    #[test]
    fn test_b(){
        let n = 2;
        let tree_input = vec![(1,2)];
        let query = vec![(1,2),(2,1)];
        let mut in_out_time: Vec<(i32,i32)> = vec![(0,0);n as usize];
        //fill child_idx array
        let mut child_idx = vec![-1; n as usize];
        let mut curr_father = 1;
        child_idx[0] = 0;
        for i in 0..n-1{
            if tree_input[i as usize].0 != curr_father{
                curr_father = tree_input[i as usize].0;
                child_idx[curr_father as usize - 1] = i;
            }
        }
        println!("child_idx : {:?}", child_idx);

        tree_pre_procces(&tree_input, &mut in_out_time, child_idx.clone(), 1, 1);
        assert_eq!(in_out_time.clone(), vec![(1,2), (2,2)]);
        assert_eq!(is_ancestor(in_out_time.clone(), query[0]), true);
        assert_eq!(is_ancestor(in_out_time.clone(), query[1]), false);
    }
    #[test]
    fn test_c(){
        let n = 3;
        let tree_input = vec![(1,3),(3,2)];
        let query = vec![(1,2),(2,1)];
        let mut in_out_time: Vec<(i32,i32)> = vec![(0,0);n as usize];
        //fill child_idx array
        let mut child_idx = vec![-1; n as usize];
        let mut curr_father = 1;
        child_idx[0] = 0;
        for i in 0..n-1{
            if tree_input[i as usize].0 != curr_father{
                curr_father = tree_input[i as usize].0;
                child_idx[curr_father as usize - 1] = i;
            }
        }
        println!("child_idx : {:?}", child_idx);

        /*tree_pre_procces(&tree_input, &mut in_out_time, child_idx.clone(), 1, 1);
        assert_eq!(in_out_time.clone(), vec![(1,2), (2,2)]);
        assert_eq!(is_ancestor(in_out_time.clone(), query[0]), true);
        assert_eq!(is_ancestor(in_out_time.clone(), query[1]), false);*/
    }
}
 