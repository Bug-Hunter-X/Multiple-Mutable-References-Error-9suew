fn main() {
    let mut x = 5;
    { //Scope of y
        let y = &mut x; 
        *y = 6; 
    }
    { //Scope of z
        let z = &mut x; 
        *z = 7; 
    }
    println!("{}", x); //Prints 7
}