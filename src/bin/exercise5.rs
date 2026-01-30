use std::io;

struct Student {
    name: String,
    score: f32,
}

impl Student {
   
    fn new(name: String, score: f32) -> Student {
        Student { name, score }
    }

    
    fn has_passed(&self) -> bool {
        self.score >= 50.0  
    }

   
    fn display_result(&self) {
        if self.has_passed() {
            println!("{} has passed the course!", self.name);
        } else {
            println!("{} has failed the course.", self.name);
        }
    }
}

fn main() {
    let mut name_input = String::new();
    let mut score_input = String::new();

    println!("Enter student's name:");
    io::stdin()
        .read_line(&mut name_input)
        .expect("Failed to read name");

  
    let name = name_input.trim().to_string();

    println!("Enter student's score:");
    io::stdin()
        .read_line(&mut score_input)
        .expect("Failed to read score");

    
    let score: f32 = match score_input.trim().parse() {
        Ok(s) => s,
        Err(_) => {
            println!("Invalid score input!");
            return;
        }
    };

    
    let student = Student::new(name, score);

   
    student.display_result();
}
