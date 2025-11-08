// Find the compound interest 

 use std::io;


 fn main() {
    loop{
     
 let mut p = String::new();
 let mut r = String::new();
 let mut t = String::new();

 println!("Please enter the value");
 io::stdin().read_line(&mut p).expect("Not a valid string");
 let p:f32 = p.trim().parse().expect("Not a valid number"); 

 println!("Please enter the value");
 io::stdin().read_line(&mut t).expect("Not a valid string");
 let t:f32 = t.trim().parse().expect("Not a valid number");

 println!("Please enter the value");
 io::stdin().read_line(&mut r).expect("Not a valid string");
 let r:f32 = r.trim().parse().expect("Not a valid number");

 // amount 
 let a:f32 = p * (1.0 + (r / 100.0)).powf(t);

 
 println!("The value is: {}",a );

 // compound interest
 let ci:f32 = a - p;
 println!("The value is: {}",ci );

 // The customers name 
 println!("Please enter customer's name");
 let mut name = String::new();
 io::stdin().read_line(&mut name).expect("Failed to read name");
 println!("The customer's name is: {}",name );

 // Write the number of items and cost
    
 let mut choice = String::new();
 println!("Do you want to redo the calculation? (y/n)");
 io::stdin().read_line(&mut choice).expect("Not making sence");
 let choice = choice.trim().to_lowercase();

 if choice == "n"{
    break;
 }
 else if choice == "y"{
    println!("restarting code");
    main();
    
 }

}
}





