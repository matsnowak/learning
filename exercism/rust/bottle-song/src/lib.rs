pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut lines: Vec<String> = Vec::new();
    for i in 0..take_down {
        lines.extend(verse(start_bottles - i));

        lines.push("\n".to_owned());
    }
    lines.concat()
}

fn verse(i: u32) -> Vec<String> {
    let mut v = Vec::<String>::new();
    v.push(format!(
        "{} green bottle{} hanging on the wall,\n",
        number(i),
        s(i)
    ));
    v.push(format!(
        "{} green bottle{} hanging on the wall,\n",
        number(i),
        s(i)
    ));
    v.push("And if one green bottle should accidentally fall,\n".to_string());
    v.push(format!(
        "There'll be {} green bottle{} hanging on the wall.\n",
        number_small(i - 1),
        s(i - 1)
    ));

    v
}

fn number(i: u32) -> String {
    match i {
        1 => "One".to_owned(),
        2 => "Two".to_owned(),
        3 => "Three".to_owned(),
        4 => "Four".to_owned(),
        5 => "Five".to_owned(),
        6 => "Six".to_owned(),
        7 => "Seven".to_owned(),
        8 => "Eight".to_owned(),
        9 => "Nine".to_owned(),
        0 => "No".to_owned(),
        10 => "Ten".to_owned(),
        _ => "Other".to_owned(),
    }
}

fn number_small(i: u32) -> String {
    number(i).to_lowercase()
}

fn s(i: u32) -> String {
    if i == 1 {
        "".to_owned()
    } else {
        "s".to_owned()
    }
}
