use std::io;  // using standard input/output 

//funtion for input
fn input() -> String {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let input1: String = input1.trim().to_string();  
    input1    //retun the function value
}

fn main() {
    println!("--- Welcome to student Information Management System ---");  //printing welcoming message
    println!("Enter your ID:");
    let id: u8 = input().parse().expect("Error, re-check it");

    println!("Enter your name:");  //name 
    let name:String = input(); 

    println!("Enter your Department:"); //department 
    let department : String = input().parse().expect("Error, re-check it");

    println!("Enter your First test score:");  //first course
    let f_score: f32 = input().parse().expect("Error, re-check it");

    println!("Enter your Second test score:");  //seceond course
    let s_score: f32 = input().parse().expect("Error, re-check it");

    println!("Enter your participation mark:");  //participation mark
    let p_mark: f32 = input().parse().expect("Error, re-check it");

    println!("Enter your exam score:");  //third course 
    let exam: f32 = input().parse().expect("Error, re-check it");


     // SUMMING STUDENT'S SCORES 
    let total = f_score + s_score + p_mark + exam;

     // CONDITIONAL STATEMENT FOR GRADING
    let grade = if total >= 70.0 {
        "A"
    } else if total >= 60.0 {
        "B"
    } else if total >= 50.0 {
        "C"
    } else if total >= 45.0 {
        "D"
    } else {
        "F"
    };

   
    //printing the details 
    println!("
        ID: {}\n 
        Name: {}\n
        Department: {}\n
        First test score: {}\n
        Second test score: {}\n
        Participation mark: {}\n
        Exam score: {}\n
        Your total score is: {}\n
        Your final grade is: {}",
        id, name, department, f_score, 
        s_score, p_mark, exam, total, grade);

     if grade == "A" {
        println!("
            'A' na water😂\n")
    } else {
        println!("
            No fear.\n")
    }
    
}