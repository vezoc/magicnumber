fn main() {
    
    print!("{}", "Random number between 0 and 255: ");
    println!("{}", gen(255));
    print!("{}", "Random number between 0 and 231: ");
    println!("{}", gen(231));
       
}

fn gen(a: u16) -> u16 {
    
    let mut val = a + 1;
    
    while val > = a + 1 {

        // initiate empty string
        let mut binstring = String::new();
        
        // looping for 8 random bits
        for _i in 0..8{
            // transforming our 0 or 1 bit into a string
            let x = magic_num::magic_number().to_string();
            // y is the binstring of the previous iteration
            let y = binstring;
            
        // adding our next random bit to the end of binstring
        binstring = [y,x].join("");
        }
        // convert binary string back into integer for comparison
        // if val is still greater than a + 1, do the loop again
        // until we get a value between 0 and a, and return it
        val = u16::from_str_radix(&binstring, 2).unwrap();
    }
    return val;

}




