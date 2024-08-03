use std::io;

fn addition(num1: i32, num2: i32) {
    println!("The sum of {} and {} is {}", num1, num2, num1 + num2);
}
fn subtraction(num1: i32, num2: i32) {
    println!("The difference of {} and {} is {}", num1, num2, num1 - num2);
}
fn multiplication(num1: i32, num2: i32) {
    println!("The multiple of {} and {} is {}", num1, num2, num1 * num2);
}
fn division(num1: i32, num2: i32) {
    println!("The diviser of {} and {} is {}", num1, num2, num1 / num2);
}


fn main() {
    let mut operator: String = String::new();
    let mut nums1: String = String::new();
    let mut nums2: String = String::new();
    println!("Please enter a literal operator(+,-,*,/): ");
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    println!("Please enter the first number: ");
    io::stdin().read_line(&mut nums1).expect("Failed to read line");
    println!("Please enter the second number: ");
    io::stdin().read_line(&mut nums2).expect("Failed to read line");
    let num1: i32 = nums1.trim().parse().expect("Invalid number");
    let num2: i32 = nums2.trim().parse().expect("Invalid number");
    let operator: &str = operator.trim(); 
    if operator == "+" {
        addition(num1, num2);
    }
    if operator == "-" {
        subtraction(num1, num2);
    }
    if operator == "*" {
        multiplication(num1, num2);
    }
    if operator == "/" {
        division(num1,num2);
    }
    if operator != "+" && operator!= "-" && operator!= "*" && operator!= "/" {
        println!("The entered character is not a literal operator.");
    }
}