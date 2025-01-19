fn main() {
    let mut x = 5;
    { //Scope of mutable borrow
        let y = &mut x;
        *y = 10; 
    }
    let z = &x; 
    println!("{}", z);
}