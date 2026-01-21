// =====================================================
// Student Grade Management System
// A realistic example demonstrating Rust basics
// =====================================================

fn main()
{
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║   UNIVERSITY STUDENT MANAGEMENT SYSTEM           ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    // -------------------------------------------------
    // CONSTANTS (Concept 3)
    // -------------------------------------------------
    const COURSE_NAME: &str = "Introduction to Computer Science";
    const COURSE_CODE: &str = "CS101";
    const MAX_STUDENTS: usize = 30;
    const PASSING_SCORE: f32 = 60.0;
    const CREDITS: u8 = 3;

    println!("Course: {} ({})", COURSE_NAME, COURSE_CODE);
    println!("Credits: {} | Max Students: {} | Passing Score: {}\n",
             CREDITS, MAX_STUDENTS, PASSING_SCORE);

    // -------------------------------------------------
    // SCALAR DATA TYPES (Concept 2)
    // Integer, Float, Boolean, Character
    // -------------------------------------------------
    println!(">>> Processing Student #1");

    let student_id: u32 = 202401;
    let age: u8 = 19;
    let midterm_score: f32 = 78.5;
    let final_score: f32 = 85.0;
    let is_enrolled: bool = true;
    let section: char = 'A';

    println!("ID: {} | Age: {} | Section: {}", student_id, age, section);
    println!("Midterm Score: {}/100 | Final Score: {}/100", midterm_score, final_score);
    println!("Enrollment Status: {}\n", if is_enrolled { "Active" } else { "Inactive" });

    // -------------------------------------------------
    // VARIABLES & MUTABILITY (Concept 1-2)
    // -------------------------------------------------
    let mut attendance_percentage: f32 = 85.0;
    println!("Initial Attendance: {:.1}%", attendance_percentage);

    attendance_percentage = 92.5; // Updated after recent classes
    println!("Updated Attendance: {:.1}%\n", attendance_percentage);

    // -------------------------------------------------
    // SHADOWING (Concept 3)
    // Demonstrating variable shadowing with type changes
    // -------------------------------------------------
    println!(">>> Score Calculation with Shadowing");

    let score = midterm_score; // Start with midterm (f32)
    println!("Step 1 - Base score: {:.1}", score);

    let score = (midterm_score * 0.4) + (final_score * 0.6); // Weighted average (f32)
    println!("Step 2 - Weighted (40%/60%): {:.1}", score);

    let score = score + 5.0; // Add participation bonus (f32)
    println!("Step 3 - With bonus: {:.1}", score);

    let score = score as i32; // Convert to integer for final grade (i32)
    println!("Step 4 - Final score: {}/100\n", score);

    // -------------------------------------------------
    // STRINGS (Concept 4)
    // Fixed-length (&str) and growable (String) strings
    // -------------------------------------------------
    println!(">>> Student Information Management");

    let university: &str = "State University"; // Fixed-length string
    let mut full_name = String::from("Alice"); // Growable string

    println!("University: {}", university);
    println!("First Name: {}", full_name);

    // String manipulation
    full_name.push_str(" Johnson");
    println!("Full Name: {}", full_name);
    println!("Name Length: {} characters", full_name.len());

    // String operations
    let email = format!("{}@university.edu", full_name.to_lowercase().replace(" ", "."));
    println!("Email: {}", email);
    println!("Valid Email: {}", email.contains("@") && email.contains("."));

    // String trimming example
    let messy_input = "  Bob Smith  ";
    println!("\nBefore trim: '{}' (length: {})", messy_input, messy_input.len());
    println!("After trim: '{}' (length: {})\n",
             messy_input.trim(), messy_input.trim().len());

    // -------------------------------------------------
    // TUPLES (Concept 5)
    // Fixed-size collections of different types
    // -------------------------------------------------
    println!(">>> Student Records (Using Tuples)");

    let student_record = ("Michael Chen", 202402, 88.5, 'B', true);

    // Accessing by index
    println!("Name: {}", student_record.0);
    println!("ID: {}", student_record.1);
    println!("Score: {:.1}/100", student_record.2);
    println!("Grade: {}", student_record.3);
    println!("Dean's List: {}", student_record.4);

    // Destructuring tuples
    let (name, id, score, grade, deans_list) = student_record;
    println!("\nDestructured: {} (ID: {}) received grade {} with score {:.1}/100",
             name, id, grade, score);

    if deans_list {
        println!("Achievement: Dean's List Honors ⭐\n");
    }

    // Nested tuples for course info
    let course_info = (
        "CS101",
        ("Sarah Williams", 202403, 92.0, 'A')
    );
    println!("Course: {} | Student: {} | Score: {:.1} | Grade: {}",
             course_info.0,
             (course_info.1).0,
             (course_info.1).2,
             (course_info.1).3);

    // Multiple variable initialization
    let (passing_count, failing_count) = (24, 3);
    println!("Class Results: {} passed, {} failed\n", passing_count, failing_count);

    // -------------------------------------------------
    // ARRAYS (Concept 5 & 6)
    // Fixed-size collections of same type
    // -------------------------------------------------
    println!(">>> Assignment Scores Tracking");

    let mut assignment_scores: [i32; 10] = [85, 90, 78, 92, 88, 95, 82, 91, 87, 94];
    println!("Assignment Scores: {:?}", assignment_scores);

    // Update a score (grade correction)
    println!("Correcting assignment 3 score...");
    assignment_scores[2] = 88;
    println!("Updated Scores: {:?}", assignment_scores);

    // Array with same initial values
    let attendance: [bool; 15] = [true; 15];
    println!("\nAttendance (15 days): {:?}", attendance);
    println!("Perfect Attendance: {}", attendance.iter().all(|&x| x));

    // Array slicing
    let first_5_assignments = &assignment_scores[0..5];
    let last_5_assignments = &assignment_scores[5..10];
    println!("\nFirst 5 Assignments: {:?}", first_5_assignments);
    println!("Last 5 Assignments: {:?}", last_5_assignments);

    // Array properties
    println!("Total Assignments: {}", assignment_scores.len());
    println!("Memory Size: {} bytes", std::mem::size_of_val(&assignment_scores));

    // Safe array access
    match assignment_scores.get(5) {
        Some(score) => println!("Assignment 6 score: {}", score),
        None => println!("Invalid assignment index"),
    }

    // String arrays
    let departments = ["Computer Science", "Mathematics", "Physics", "Engineering", "Biology"];
    println!("\nAvailable Departments:");
    for (index, dept) in departments.iter().enumerate() {
        println!("  {}. {}", index + 1, dept);
    }

    // Number formatting
    let total_enrolled = 1_245;
    let budget = 50_000_000;
    println!("\nTotal Students Enrolled: {}", total_enrolled);
    println!("Annual Budget: ${}\n", budget);

    // -------------------------------------------------
    // FUNCTIONS (Concept 7)
    // -------------------------------------------------
    println!(">>> Grade Processing Functions\n");

    // Basic function
    print_separator();

    // Function with parameters
    display_student_info("Emma Davis", 202405, 'B');

    // Function with return value
    let avg = calculate_average(85.0, 90.0, 88.0);
    println!("Average Score: {:.2}/100\n", avg);

    // Function with multiple return values
    let (sum, average, count) = calculate_statistics(&assignment_scores);
    println!("Assignment Statistics:");
    println!("  Total: {} points", sum);
    println!("  Average: {:.2}/100", average);
    println!("  Count: {} assignments", count);
    println!("  Highest Possible: {} points\n", count * 100);

    // -------------------------------------------------
    // CODE BLOCKS (Concept 7)
    // -------------------------------------------------
    let semester_summary = {
        let completed_courses = 5;
        let total_credits = 15;
        let gpa = 3.65;
        format!("Completed: {} courses | Credits: {} | GPA: {:.2}",
                completed_courses, total_credits, gpa)
    };
    println!("Semester Summary: {}\n", semester_summary);

    // -------------------------------------------------
    // REALISTIC GRADE CALCULATION
    // -------------------------------------------------
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║          FINAL GRADE CALCULATION                  ║");
    println!("╚═══════════════════════════════════════════════════╝\n");

    let student_name = "James Rodriguez";
    let student_num = 202410;

    // Real assessment scores
    let quiz_scores = [78.0, 85.0, 92.0, 88.0, 90.0];
    let midterm = 82.0;
    let final_exam = 89.0;
    let project = 95.0;

    println!("Student: {} (ID: {})", student_name, student_num);
    println!("Quiz Scores: {:?}", quiz_scores);
    println!("Midterm Exam: {}/100", midterm);
    println!("Final Exam: {}/100", final_exam);
    println!("Project: {}/100\n", project);

    let (final_score, letter_grade, status) = calculate_final_grade(
        &quiz_scores, midterm, final_exam, project, PASSING_SCORE
    );

    println!("═══════════════════════════════════════════════════");
    println!("FINAL SCORE: {:.2}/100", final_score);
    println!("LETTER GRADE: {}", letter_grade);
    println!("STATUS: {}", status);
    println!("═══════════════════════════════════════════════════\n");

    // Grade distribution display
    display_grade_distribution();

    // -------------------------------------------------
    // NUMBER FORMAT CONVERSIONS (Concept 3)
    // -------------------------------------------------
    println!(">>> Number Representations");
    let student_code = 255;
    println!("Student Code Formats:");
    println!("  Decimal: {}", student_code);
    println!("  Binary: {:08b}", student_code);
    println!("  Octal: {:o}", student_code);
    println!("  Hexadecimal: {:X}\n", student_code);

    // -------------------------------------------------
    // SYSTEM SUMMARY
    // -------------------------------------------------
    let enrolled_students = 27;
    generate_course_report(enrolled_students, MAX_STUDENTS, PASSING_SCORE);
}

// ═════════════════════════════════════════════════════
// FUNCTION DEFINITIONS
// ═════════════════════════════════════════════════════

/// Prints a simple separator line
fn print_separator() {
    println!("───────────────────────────────────────────────────");
}

/// Displays basic student information
fn display_student_info(name: &str, id: u32, grade: char) {
    println!("Student: {} | ID: {} | Grade: {}", name, id, grade);
}

/// Calculates the average of three scores
fn calculate_average(score1: f32, score2: f32, score3: f32) -> f32 {
    (score1 + score2 + score3) / 3.0
}

/// Returns sum, average, and count of assignment scores
fn calculate_statistics(scores: &[i32; 10]) -> (i32, f32, usize) {
    let sum: i32 = scores.iter().sum();
    let count = scores.len();
    let average = sum as f32 / count as f32;

    (sum, average, count)
}

/// Calculates final grade based on weighted components
/// Weights: Quizzes 20%, Midterm 25%, Final 35%, Project 20%
fn calculate_final_grade(
    quizzes: &[f32],
    midterm: f32,
    final_exam: f32,
    project: f32,
    passing: f32
) -> (f32, char, String) {

    // Calculate quiz average
    let quiz_sum: f32 = quizzes.iter().sum();
    let quiz_avg = quiz_sum / quizzes.len() as f32;

    // Weighted calculation
    let final_score = (quiz_avg * 0.20) + (midterm * 0.25) +
                      (final_exam * 0.35) + (project * 0.20);

    // Determine letter grade
    let letter_grade = if final_score >= 90.0 {
        'A'
    } else if final_score >= 80.0 {
        'B'
    } else if final_score >= 70.0 {
        'C'
    } else if final_score >= 60.0 {
        'D'
    } else {
        'F'
    };

    // Determine status
    let status = if final_score >= 90.0 {
        String::from("EXCELLENT - Dean's List")
    } else if final_score >= passing {
        String::from("PASSED")
    } else {
        String::from("FAILED - Retake Required")
    };

    (final_score, letter_grade, status)
}

/// Displays grade distribution for the class
fn display_grade_distribution() {
    println!(">>> Class Grade Distribution");

    let grade_counts = [
        ('A', 8),
        ('B', 12),
        ('C', 5),
        ('D', 2),
        ('F', 1),
    ];

    let total_students = 28;

    for (grade, count) in &grade_counts {
        let percentage = (*count as f32 / total_students as f32) * 100.0;
        let bar = "█".repeat(*count as usize);
        println!("  Grade {}: {:2} students ({:5.1}%) {}",
                 grade, count, percentage, bar);
    }
    println!();
}

/// Generates a comprehensive course report
fn generate_course_report(enrolled: usize, capacity: usize, passing: f32) {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║            COURSE SUMMARY REPORT                  ║");
    println!("╚═══════════════════════════════════════════════════╝");

    let available = capacity - enrolled;
    let utilization = (enrolled as f32 / capacity as f32) * 100.0;

    println!("Enrolled Students: {}/{}", enrolled, capacity);
    println!("Available Seats: {}", available);
    println!("Capacity: {:.1}%", utilization);
    println!("Passing Score Required: {}/100", passing);

    let status = if available == 0 {
        "FULL - Waitlist Available"
    } else if available <= 3 {
        "Almost Full - Register Soon"
    } else {
        "Open for Enrollment"
    };

    println!("Enrollment Status: {}", status);
    println!("═══════════════════════════════════════════════════");
}
