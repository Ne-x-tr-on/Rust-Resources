#[derive(Debug, Clone)]
struct Student {
    name: String,
    math: u32,
    english: u32,
    total: u32,
}

fn main() {
    let students = vec![
        Student { name: "Newton".to_string(), math: 80, english: 90, total: 0 },
        Student { name: "Alice".to_string(), math: 85, english: 95, total: 0 },
        Student { name: "Bob".to_string(), math: 78, english: 88, total: 0 },
    ];

    // Compute total marks using map + closure
    let students_with_total: Vec<Student> = students.into_iter()
        .map(|mut s| {
            s.total = s.math + s.english; // closure modifies each student
            s
        })
        .collect();

    println!("Students with totals: {:?}", students_with_total);

    let mut ranked_students = students_with_total.clone();

// Sort descending by total
ranked_students.sort_by(|a, b| b.total.cmp(&a.total));

for (pos, student) in ranked_students.iter().enumerate() {
    println!("Position {}: {} with total {}", pos+1, student.name, student.total);
}

}
