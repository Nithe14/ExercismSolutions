pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        x => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", x, x, x-1)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: String = "".to_string();
    if end >= start {
        for n in start..=end {
            song.push_str(&verse(n));
            song.push_str("\n");
        }
    } else {
        for n in (end..=start).rev() {
            song.push_str(&verse(n));
            song.push_str("\n");
        }
    }
    song.pop(); //remove last \n char
    song
}
