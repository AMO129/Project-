use std::io;  // using standard input/output 

//funtion for input
fn input() -> String {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let input1: String = input1.trim().to_string().to_lowercase();  
    input1    //retun the function value
}

fn main() {
     
     // VECTOR FOR STORING DATA
    let mut student_list: Vec<String> = Vec::new();

    println!("--- Welcome to student Information Management System ---");  //printing welcoming message
    

    
        println!("\n ---- Enter New Student's Details ----");

        println!("Enter your ID:");
        let id: u8 = input().parse().expect("Error, re-check it");

        println!("Enter your name:");  //name 
        let name:String = input(); 

    
        println!("Enter your Department:"); //department 
        let department : String = input().parse().expect("Error, re-check it");

    
    loop{
        println!("Enter your course code:"); //department 
        let cousre_code : String = input().parse().expect("Error, re-check it");


        println!("Enter your First test score:");  //first course
        let f_score: f32 = input().parse().expect("Error, re-check it");

        println!("Enter your Second test score:");  //seceond course
        let s_score: f32 = input().parse().expect("Error, re-check it");

        println!("Enter your participation mark:");  //participation mark
        let p_mark: f32 = input().parse().expect("Error, re-check it");

        println!("Enter your exam score:");  //third course 
        let exam: f32 = input().parse().expect("Error, re-check it");

        // CONFIRMING USER INPUTS
        println!("\nYou entered: {} ID: {}", name, id);
        println!("Is this correct? (y to save & n to discard):");

        let confirm = input().to_lowercase();

        if confirm == "y"{
            student_list.push(name.clone());
            println!("Student saved.");
        } else {
            println!("Entry discarded");
            continue;
        }

       

    
         // SUMMING STUDENT'S SCORES 
        let total = f_score + s_score + p_mark + exam;

         // CONDITIONAL STATEMENT FOR GRADING
        let grade = if total <= 100.0 && total >= 70.0{
            "A"
        } else if total <= 69.9 && total >= 60.0 {
            "B"
        } else if total <= 59.9 && total >= 50.0 {
            "C"
        } else if total <= 45.9 && total >= 45.0 {
            "D"
        } else {
            "F"
        };

    
       

   
    //printing the details 
    println!("
        ID: {}\n 
        Name: {}\n
        Department: {}\n
        Course: {}\n
        First test score: {}\n
        Second test score: {}\n
        Participation mark: {}\n
        Exam score: {}\n
        Your total score is: {}\n
        Your final grade is: {}",
        id, name, department, cousre_code, f_score, 
        s_score, p_mark, exam, total, grade);

     if grade == "A" {
        println!("
            'A' na water😂\n")
    } else {
        println!("
            No fear.\n")
    }

     // ASKING FOR OTHER STUDENTS DETAILS

        println!("Do you want to continue? (yes/no)");

        let cont = input().to_lowercase();

        if cont =="no"{
            break;
        }

  }
    
}
