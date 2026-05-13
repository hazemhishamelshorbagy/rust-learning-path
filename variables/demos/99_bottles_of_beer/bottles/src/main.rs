fn main() {
    bottles_of_beer();
}

fn bottles_of_beer() {
    for i in (0..100).rev() {
        if i == 0 {
            println!("No more bottles of beer on the wall, no more bottles of beer.");
            println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
        } else if i == 1 {
            println!("{} bottle of beer on the wall, {} bottle of beer.", i, i);
            println!("Take one down and pass it around, no more bottles of beer on the wall.");
        } else {
            println!("{} bottles of beer on the wall, {} bottles of beer.", i, i);
            println!(
                "Take one down and pass it around, {} bottle{} of beer on the wall.",
                i - 1,
                if i - 1 == 1 { "" } else { "s" }
            );
        }
    }
}
