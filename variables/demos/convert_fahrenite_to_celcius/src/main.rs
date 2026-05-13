fn main() {
    println!("Hello, world!");
    let ferhenite = 100.0;
    let celcius = convert_fahrenite_to_celcius(ferhenite);
    println!("{} ferhenite is {} celcius", ferhenite, celcius);
}
fn convert_fahrenite_to_celcius(ferhenite:f64)->f64{
    (ferhenite - 32.0) * 5.0 / 9.0
}
// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.