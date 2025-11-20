fn main() {

    // initialize a multiple tuple
    let mut mountain_heights= ("Everest", 8848, "Fishtail", 6993);


    println!("Original tuple = {:?}", mountain_heights);

    // change 3rd annd 4th element of a multiple tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}",mountain_heights );
}
