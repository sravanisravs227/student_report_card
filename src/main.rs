use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn calculate_average(total_marks: f64, num_subjects: f64) -> f64 {
    total_marks / num_subjects
}

fn assign_grade(avg: f64) -> &'static str {
    if avg >= 90.0 {
        "A"
    } else if avg >= 75.0 {
        "B"
    } else if avg >= 60.0 {
        "C"
    } else {
        "D"
    }
}

fn main() {
    let student_name = "Sravani";
    let total_marks = 450.0;
    let num_subjects = 5.0;

    let avg = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(avg);

    // PDF setup
    let (doc, page1, layer1) = PdfDocument::new("Student Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();

    let report = format!(
        "Student Name: {}\nTotal Marks: {}\nNumber of Subjects: {}\nAverage: {:.2}\nGrade: {}",
        student_name, total_marks, num_subjects, avg, grade
    );

    current_layer.use_text(report, 14.0, Mm(20.0), Mm(270.0), &font);

    let mut file = BufWriter::new(File::create("report_card.pdf").unwrap());
    doc.save(&mut file).unwrap();

    println!("PDF report card generated successfully!");
}
