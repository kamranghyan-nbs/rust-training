fn main() {
    let a: u8 = 10;
    let b = &a;
    let c = &b;

    println!("a:{}, b:{}, c:{}", a, b, c);

    println!("The address of a is {:p}", b);
    println!("The address of b is {:p}", c);
}
