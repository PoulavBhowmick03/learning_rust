// fn main(){
//     let mut string :String = String::from("hello world");
//     string.push_str("what's up");
//     println!("{}", string);

// }

// fn main() {
//     let var_name = ("Poulav",20);
//     let emp_info: (&str, u8) = var_name;

//     let emp_name = emp_info.0;
//     let emp_age = emp_info.1;
//     let (employee_name, employee_age) = emp_info;
//     println!("Name: {} Age: {}", emp_name, emp_age);
//     println!("Name: {} Age: {}", employee_name, employee_age);
// }

fn main() {
    let a:i32 = 100;
    let b:i32 = 200;
    let result = sum(a,b);
    println!("Sum of {} and {} is {}", a, b, result);
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}
