use std::io::Write;

fn input() -> String {
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Not a valid input");
    let input1: String = input1.trim().to_string().to_lowercase();  
    input1    
}

fn main() {
    let mut student_list: Vec<String> = Vec::new();

    let record_to_save = student_list.join("\n");

    println!("\n--- SIMS is now online😁 ---");


    'student_loop: loop {
        println!("\n ---- Enter New Student's Details ----");


        /*println!("\n Before then please note that you can exit this program at any point by typing stop.
            \nPress Enter key on your keyboard to confirm you understand that or continue");

        if input() == "stop"{
            println!("You have prompted to stop the program
                \n Do you really want to exit the program? (y to stop & n to continue)");
            if input() != "y"{
                continue;
            }
        }*/

        println!("Enter your Matriculation Number:");
        let id: String = input();

        println!("Enter your name:");
        let name: String = input(); 

        println!("Enter your Department:");
        let department: String = input(); // Fixed: parse() removed as Dept is a String

         // CONFIRMING USER INPUTS
            println!("\nYou entered:
                Name: {}\n
                Matriculation Number: {}\n
                Department: {}",
                name, id, department);
            
            println!("\nAre those your correct details? (y to save & n to discard):");
            if input() != "y" {
                println!("Entry discarded, try those course details again.");
                continue 'student_loop; 
            }

        'course_loop: loop {
            println!("\n{}'s details (Matriculation Number: {})", name, id);
            println!("Enter your course code:");
            let course_code: String = input();

            println!("Enter your First test score:");
            let f_score: f32 = input().parse().expect("Error, re-check it");

            println!("Enter your Second test score:");
            let s_score: f32 = input().parse().expect("Error, re-check it");

            println!("Enter your participation mark:");
            let p_mark: f32 = input().parse().expect("Error, re-check it");

            println!("Enter your exam score:");
            let exam: f32 = input().parse().expect("Error, re-check it");

            // CONFIRMING USER INPUTS
            println!("\nYou entered:
                Course code: {}\n
                First test scores: {}\n
                Second test score: {}\n
                participation mark: {}\n
                Exam score: {}", 
                course_code, f_score, s_score, p_mark, exam);
            
            println!("\nAre those your correct details? (y to save & n to discard):");
            if input() != "y" {
                println!("Entry discarded, try those course details again.");
                continue 'course_loop; 
            }

            student_list.push(name.clone());
            println!("Student data recorded.");

            let total = f_score + s_score + p_mark + exam;
            let grade = if total <= 100.0 && total >= 70.0 { "A" } 
                        else if total <= 69.0 && total >= 60.0 { "B" } 
                        else if total <= 59.9 && total >= 50.0 { "C" } 
                        else if total <= 49.9 && total >= 45.0 { "D" } 
                        else if total <=44.9 && total >= 0.0 { "F" }
                        else {"total score is out of scope"};

            // ASKING FOR MORE RECORDS
            println!("\nDo you want to enter another course's records for this student? (y/n)");
            if input() == "y" {
                continue 'course_loop; // Restarts at Course Code
            } else {

                println!("
                    Final Grade for {}: is '{}'\n 
                    (Total: {})\n", 
                    course_code, grade, total);

                if grade == "A" { println!("'A' na water😂\n"); } 
                else { println!("No fear.\n"); }

                println!("\nDo you want to enter a different student's records? (y/n)");
                if input() == "y" {
                    continue 'student_loop; // Restarts at ID/Name
                } else {
                    println!("We have come to the end, your data has been updated on the database😁");
                    break 'student_loop; // Kills the program
                }
            }

        } // End of course_loop
    } // End of student_loop

    let mut file = std::fs::File::create("SIMS.csv").expect("failed to create");
    file.write_all(record_to_save.as_bytes()).expect("failed to write");
    println!("\nRecords saved successfully");
}
