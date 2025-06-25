use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // normal simple solution
    // input.chars().rev().collect()

    // iterator with str push
    // let mut buff = String::with_capacity(input.chars().count());
    //
    // for c in input.chars().rev() {
    //     buff.push(c);
    // }
    //
    // buff

    // graphemes
    input.graphemes(true).rev().collect()
}
