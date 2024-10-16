// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

// I AM DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string()); // to_string() transform &str into String
    string(String::from("hi")); // create new String from &str
    string("rust is fun!".to_owned());
    string_slice("nice weather".into()); // "nice weather" is &str and 'into()' transforms it into String
    string(format!("Interpolation {}", "Station")); // format! returns String
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim()); // remove \n \t " " and returns &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // replace returns String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
