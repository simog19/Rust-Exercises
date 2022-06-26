use clock::Clock;
fn main() {
    let mut c = Clock::new(0,10);
    //c = c.add_minutes(-20);
    let c1 = Clock::new(3,10);
    let mut clock= Clock::new(2,20).add_minutes(-3000);
    c=c+c1;
    println!("{} --- {}",c, clock);
    clock=clock-c;
    println!("{} ", clock);
}
