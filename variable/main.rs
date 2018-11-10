fn main() {
    // warning: value assigned to `a` is never read
    let (mut a: i32, b: i32) = (0, 0);
    let c: u8 = 'c';

    a = 10;

    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
}
