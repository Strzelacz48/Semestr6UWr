use std::vec;

fn main() {
    println!("Hello, world!");
}
fn heap_init(vec: Vec<i32>,max: bool) -> Vec<i32> {
    let mut v = vec;
    for i in (0..v.len()).rev() {
        if i*2+1 < v.len() {
            if max {
                if v[i] < v[i*2+1] {
                    let temp = v[i];
                    v[i] = v[i*2+1];
                    v[i*2+1] = temp;
                }
            } else {
                if v[i] > v[i*2+1] {
                    let temp = v[i];
                    v[i] = v[i*2+1];
                    v[i*2+1] = temp;
                }
            }
        }
    }
    println!("{:?}", v);
    vec
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}