fn main() {
    let v: Vec<i32> = vec![1, 2];
    let it1 = MyCycle::new(v.iter(), 2);
    let it2 = MyCycle::new(v.iter(), 2);

    println!("{:?}", it1.zip(it2).next().unwrap());
}
