fn main() {

    // initialiization of tuple  with data type
    let datatype_tuple: (&str,f32,u8) = ("Rust", 3.24, 100);
    println!("Tuple  contents = {:?}", datatype_tuple);

    // initialization of tuple without data  type 
    let no_datatype_tuple = ("Rust", "fun", 100);
    println!("Tuple contents = {:?}", no_datatype_tuple );

    // acceessing tuple elements at index 0
    println!("value at Index 0 = {}",datatype_tuple.0 );

    // accessing tuple elements at index 1
    println!("value at Index 1 = {}",datatype_tuple.1 );

    // acceessing tuple elements at index 2
    println!("value at Index 2 = {}",datatype_tuple.2 );  
}
