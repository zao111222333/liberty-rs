use super::*;

#[test]
fn it_works() {
    let mut timing = Timing::default();
    let pin_a = Pin::default();
    let mut pin_b = Pin::default();
    timing.related_pin = vec![&pin_a];
    pin_b.timing_list = vec![timing];
    println!("{:?}", pin_b);
    // println!("{:?}", pin_a);
}