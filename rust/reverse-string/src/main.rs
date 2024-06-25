use unicode_segmentation::UnicodeSegmentation;

fn reverse(input: &str) -> String {
    let mut string = String::new();

    let graphemes: Vec<&str> = input.graphemes(true).collect();

    for i in (0..input.len()).rev() {
        if let Some(ch) = graphemes.get(i) {
            string.push_str(ch);
        } else {
            println!("Failed to reverse a string");
        }
    }

    string
}

fn main() {
    let s = reverse("Sanjai is");
    println!("{s}");
}
