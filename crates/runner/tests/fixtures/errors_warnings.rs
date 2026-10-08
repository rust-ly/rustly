fn main() {
    let unused = 5;
    let s = String::from("hi");
    let t = s;
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("{s} {first}");
}
