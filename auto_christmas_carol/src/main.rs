fn main() {

    println!("Lyrics to The Twelve Days of Christmas Carol!\n");

    const DYNAMIC_LYRICS: [&str;12] = [
        "And a song for the Christmas tree",
        "Two candy canes",
        "Three boughs of holly",
        "Four colored lights",
        "A shining star",
        "Little silver bells",
        "Candles a-glowing",
        "Gold and silver tinsel",
        "A guardian angel",
        "Some mistletoe",
        "Gifts for one and all",
        "All their good wishes"];

    const DAYS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
            "tenth", "11th", "12th"];


    loop_through_carol(DYNAMIC_LYRICS, DAYS)
}

fn loop_through_carol(lyrics: [&str;12], days_list: [&str;12]) {
    for i in 0..12 {
        match i {
            0 => {
                println!("\nOn the {} day of Christmas", days_list[i]);
                println!("My good friends brought to me\nA song and a Christmas tree");
            }
            _ => {
                println!("\nOn the {} day of Christmas", days_list[i]);
                println!("My good friends brought to me\n{}", lyrics[i]);
                for j in (0..i).rev() {
                    println!("{}", lyrics[j]);
                }
            }
        }
    }
}
