struct Student{
    major: String,
}

//high order 
fn update(collection: &mut Vec<Student>, operation: fn(&mut Student, String), major: String)
{
    for student in collection.iter_mut() {
        operation(student, major.clone());
    }
}
//first order functions, assign_major(student, major_declared)
fn assign_major(student: &mut Student, major_declared: String) {
    student.major = major_declared;

}

// Helper function to print student majors
fn print_majors(students: &[Student]) {
    print!("Majors: ");
    for (i, student) in students.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", student.major);
    }
    println!();
}

//vector of students 1,2,3; update majors

fn main() {
    let mut stu = vec![
        Student { major: String::from("Engineering") },
        Student { major: String::from("Science") },
        Student { major: String::from("") },
    ];

    print!("Original ");
    print_majors(&stu);
    
    // Update majors
    let new_major = String::from("Computer Science");
    update(&mut stu, assign_major, new_major);
    
    print!("After updating majors: ");
    print_majors(&stu);
}
