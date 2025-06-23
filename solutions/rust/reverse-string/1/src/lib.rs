use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();

    let mut chars = input.graphemes(true).rev();

    while let Some(c) = chars.next() {
        reversed.push_str(c);
    }

    reversed
}
