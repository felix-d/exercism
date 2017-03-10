pub fn bottles(num: i32, capitalize: bool) -> String {
    match num {
        0 => (if capitalize { "No more bottles" } else { "no more bottles" }).to_owned(),
        1 => "1 bottle".to_owned(),
        _ => format!("{} bottles", num)
    }
}

pub fn last_part(num: i32) -> String {
    let new_num: i32 = ((num - 1 % 100) + 100) % 100;
    match num {
        0 => format!("Go to the store and buy some more, {} of beer on the wall.\n", bottles(new_num, false)),
        1 => format!("Take it down and pass it around, {} of beer on the wall.\n", bottles(new_num, false)),
        _ => format!("Take one down and pass it around, {} of beer on the wall.\n", bottles(new_num, false))
    }
}

pub fn verse(num: i32) -> String {
    format!(
        "{} of beer on the wall, {} of beer.\n{}",
        bottles(num, true),
        bottles(num, false),
        last_part(num)
    )
}

pub fn sing(num: i32, num2: i32) -> String {
    let mut result: String = "".to_owned();

    for i in (num2..num + 1).rev() {
        if i == num {
            result.push_str(&format!("{}", verse(i)));
        } else {
            result.push_str(&format!("\n{}", verse(i)));
        }
    }
    result
}
