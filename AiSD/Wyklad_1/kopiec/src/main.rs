fn main() {
    println!("Hello, world!");
}
fn heap() {
    let v = vec![1, 2, 3];
    let v2 = &v;
    println!("{:?}", v);
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}