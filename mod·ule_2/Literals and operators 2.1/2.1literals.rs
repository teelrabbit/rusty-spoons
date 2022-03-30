//
//
//
//
//notes for literals
//
fn main() { //making a main function 
    //adding integers while using a 32byte signed integer
    println!("1 + 2 = {} example using the suffix i32 to indicate a 32bit unsigned integer", 1u32 + 2);
    println!("1 - 2 = {} example using the suffix u32 to indicate a 32 bit signed integer", 1i32 - 2);
    //note that a signed int has a negitive range while a unsigned it can only have a positive
    //range
    //
        // Short-circuiting boolean logic
    println!("true AND false is {}", true && false); // will return false
    println!("true OR false is {}", true || false); // will return true
    println!("NOT true is {}", !true); // will return false

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
