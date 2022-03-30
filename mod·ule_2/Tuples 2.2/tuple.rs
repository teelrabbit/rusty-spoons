//
//
//
// section on tuples
// tuples can be used inRUST to group multiple varibles in one space
//
fn main() {
    let tuple1 = (20, 25, 30, 35);
    // above is a tuple named "tuple1" that holds all integers
    // "let" is used when defining tuples and then ()

    println!("Index 2 of the tuple created is {}", tuple1.2);
    // In that print statment the tuple is called and then the index of the number inside the tuple
    // tuples can also be used to define other tuples, such as in the example bellow

    let (a, b, c, d) = tuple1;
    println!("Pinting A {}", a);
    println!("Pinting B {}", b);
    println!("Pinting C {}", c);
    println!("Pinting D {}", d);
}   
