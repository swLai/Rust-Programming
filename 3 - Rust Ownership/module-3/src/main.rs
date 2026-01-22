// ===================================================
// Student Grade Management System
// Demonstrates: Ownership, References, and Memory
// ===================================================

fn main() {
    println!("=== Student Grade Management System ===\n");

    // -------------------------------------------
    // 1. OWNERSHIP RULES
    // - Each value has one owner
    // - Only one owner at a time
    // - Value dropped when owner goes out of scope
    // -------------------------------------------
    println!("1. OWNERSHIP RULES:");
    {
        let student_name = String::from("Alice Johnson");
        println!("   Student: {}", student_name);
    } // student_name goes out of scope and is dropped here
    // println!("{}", student_name); // ❌ Error: out of scope

    println!("   ✓ Value dropped when owner goes out of scope\n");

    // -------------------------------------------
    // 2. PRIMITIVE vs NON-PRIMITIVE TYPES
    // -------------------------------------------
    println!("2. PRIMITIVE vs NON-PRIMITIVE TYPES:");

    // Primitive types (i32, f64, bool) - Stack allocated, COPIED
    let score1: i32 = 95;
    let score2 = score1;  // Copy happens
    println!("   Primitive (i32): score1={}, score2={}", score1, score2);
    println!("   ✓ Both valid - primitive types are copied\n");

    // Non-primitive types (String, Vec) - Heap allocated, MOVED
    let name1 = String::from("Bob Smith");
    let name2 = name1;  // Ownership moves to name2
    println!("   Non-primitive (String): name2={}", name2);
    // println!("{}", name1); // ❌ Error: name1 was moved
    println!("   ✓ Original invalid - ownership transferred\n");

    // -------------------------------------------
    // 3. REFERENCES (Borrowing) - Immutable
    // -------------------------------------------
    println!("3. IMMUTABLE REFERENCES:");
    let subject = String::from("Mathematics");

    // Create references with &
    let ref1 = &subject;
    let ref2 = &subject;

    println!("   Original: {}", subject);
    println!("   Reference 1: {}", ref1);
    println!("   Reference 2: {}", ref2);
    println!("   ✓ Multiple immutable references allowed\n");

    // -------------------------------------------
    // 4. OWNERSHIP and FUNCTIONS
    // -------------------------------------------
    println!("4. OWNERSHIP and FUNCTIONS:");

    // Stack function - primitive types are copied
    let grade_num = 88;
    print_grade(grade_num);
    println!("   After function, grade_num still valid: {}", grade_num);
    println!("   ✓ Primitives are copied to functions\n");

    // Heap function - without reference (ownership moves)
    let student1 = String::from("Charlie Brown");
    print_student_name_move(student1);
    // println!("{}", student1); // ❌ Error: student1 was moved
    println!("   ✓ Ownership transferred to function\n");

    // Using reference to keep ownership
    let student2 = String::from("Diana Prince");
    print_student_name_borrow(&student2);
    println!("   After function, student2 still valid: {}", student2);
    println!("   ✓ Reference allows keeping ownership\n");

    // -------------------------------------------
    // 5. MUTABLE REFERENCES
    // -------------------------------------------
    println!("5. MUTABLE REFERENCES:");
    let mut grades = vec![85, 90, 78, 92];
    println!("   Original grades: {:?}", grades);

    // Pass mutable reference to modify
    add_bonus_points(&mut grades, 5);
    println!("   After bonus: {:?}", grades);
    println!("   ✓ Mutable reference can modify data\n");

    // -------------------------------------------
    // 6. REFERENCE RULES
    // - Only ONE mutable reference in a scope
    // - OR many immutable references
    // - Cannot mix mutable and immutable
    // -------------------------------------------
    println!("6. REFERENCE RULES:");

    let mut scores = vec![70, 80, 90];

    // Multiple immutable references OK
    let r1 = &scores;
    let r2 = &scores;
    println!("   Multiple immutable refs: {:?}, {:?}", r1, r2);
    // r1 and r2 go out of scope here

    // One mutable reference OK (after immutable ones are done)
    let r3 = &mut scores;
    r3.push(100);
    println!("   Mutable ref modified: {:?}", r3);
    println!("   ✓ Reference rules enforced\n");

    // -------------------------------------------
    // 7. DEREFERENCING
    // -------------------------------------------
    println!("7. DEREFERENCING:");

    let mut total_score = 250;
    let score_ref = &mut total_score;

    // Use * to access the value
    let copy_of_score = *score_ref;
    println!("   Dereferenced value: {}", copy_of_score);

    // Use * to modify through reference
    *score_ref = 300;
    println!("   Modified via dereference: {}", total_score);
    println!("   ✓ * operator accesses referenced value\n");

    // -------------------------------------------
    // 8. CLONING for Independent Copies
    // -------------------------------------------
    println!("8. CLONING:");

    let original_grades = vec![88, 92, 85];
    let cloned_grades = original_grades.clone();

    println!("   Original: {:?}", original_grades);
    println!("   Clone: {:?}", cloned_grades);
    println!("   ✓ Both vectors are independent\n");

    // -------------------------------------------
    // 9. PRACTICAL EXAMPLE: Grade Calculator
    // -------------------------------------------
    println!("9. PRACTICAL EXAMPLE:");

    let student_name = String::from("Emma Wilson");
    let mut exam_scores = vec![85, 90, 88, 92];

    // Display using immutable reference
    display_student_info(&student_name, &exam_scores);

    // Calculate average using immutable reference
    let avg = calculate_average(&exam_scores);
    println!("   Average score: {:.2}", avg);

    // Add extra credit using mutable reference
    add_extra_credit(&mut exam_scores, 3);
    println!("   After extra credit: {:?}", exam_scores);

    let new_avg = calculate_average(&exam_scores);
    println!("   New average: {:.2}", new_avg);

    println!("   Student name still valid: {}", student_name);
    println!("   ✓ References preserve ownership\n");

    println!("=== Program Complete ===");
}

// -------------------------------------------
// HELPER FUNCTIONS
// -------------------------------------------

// Takes primitive type - value is copied
fn print_grade(grade: i32) {
    println!("   Grade received in function: {}", grade);
}

// Takes ownership - original becomes invalid
fn print_student_name_move(name: String) {
    println!("   Student name (ownership moved): {}", name);
    // name is dropped when function ends
}

// Borrows with immutable reference
fn print_student_name_borrow(name: &String) {
    println!("   Student name (borrowed): {}", name);
}

// Mutable reference to modify data
fn add_bonus_points(grades: &mut Vec<i32>, bonus: i32) {
    let mut i = 0;
    while i < grades.len() {
        grades[i] = grades[i] + bonus;
        i += 1;
    }
}

// Immutable reference for read-only access
fn display_student_info(name: &String, scores: &Vec<i32>) {
    println!("   Student: {}", name);
    println!("   Scores: {:?}", scores);
}

// Calculate average using immutable reference
fn calculate_average(scores: &Vec<i32>) -> f64 {
    let mut sum = 0;
    let mut i = 0;
    while i < scores.len() {
        sum += scores[i];
        i += 1;
    }
    sum as f64 / scores.len() as f64
}

// Add extra credit using mutable reference
fn add_extra_credit(scores: &mut Vec<i32>, extra: i32) {
    let mut i = 0;
    while i < scores.len() {
        scores[i] += extra;
        i += 1;
    }
}
