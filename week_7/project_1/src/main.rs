use std::io;

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<f64>().expect("Invalid number")
}

fn area_trapezium() {
    let height = read_input("Enter height:");
    let base1 = read_input("Enter base1:");
    let base2 = read_input("Enter base2:");
    let area = (height / 2.0) * (base1 + base2);
    println!("Area of Trapezium = {}", area);
}

fn area_rhombus() {
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}", area);
}

fn area_parallelogram() {
    let base = read_input("Enter base:");
    let altitude = read_input("Enter altitude:");
    let area = base * altitude;
    println!("Area of Parallelogram = {}", area);
}

fn area_cube() {
    let side = read_input("Enter side of cube:");
    let area = 6.0 * side.powi(2);
    println!("Area of Cube = {}", area);
}

fn volume_cylinder() {
    let radius = read_input("Enter radius:");
    let height = read_input("Enter height:");
    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("Volume of Cylinder = {}", volume);
}

fn main() {
    println!("Select a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choice");

    match choice.trim() {
        "1" => area_trapezium(),
        "2" => area_rhombus(),
        "3" => area_parallelogram(),
        "4" => area_cube(),
        "5" => volume_cylinder(),
        _ => println!("Invalid choice"),
    }
}

