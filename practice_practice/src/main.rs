use std::io;

fn main() {
    let mut first_grade= String::new();  
    let mut second_grade = String::new();
    let mut third_grade = String::new();

   //  Enter your name
    println!("Please enter your name ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");
    println!("The name is: {}",name );


    // Input your values 
    println!("Please enter the value");
    io::stdin().read_line(&mut first_grade).expect("Not a valid string");
    let first_grade:f32 = first_grade.trim().parse().expect("Not a valid number");

    println!("Please  enter your value");
    io::stdin().read_line(&mut second_grade).expect("Not a valid string");
    let second_grade:f32 = second_grade.trim().parse().expect("Not a valid number");

    println!("Plese enter the value");
    io::stdin().read_line(&mut third_grade).expect("Not a valid string");
    let third_grade:f32 = third_grade.trim().parse().expect("Not a valid number");

    // Find the average in 2 decimal places
    let  avg:f32 = (first_grade + &second_grade + &third_grade) / 3.0 ;
    println!("The average of the scores is: {}", avg);

    if avg >= 70.0 {
    println!("The grade is: ", );
    }
    else if avg > 60.0 {
        println!("The grade is: ", );
    }
    else if avg  > 50.0 {
        println!("The grade is: ", );
    }
    else if avg  > 45.0 {
        println!("The grade is:", );
    }
    else if avg > 0.0 {
        println!("The grade is:", );
    }



 
}