// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

// I AM DONE

fn main() {
    let mut res = 42;
    let mut option = Some(0);
   
    while let Some(mut x) = option{
        if x < 12 {
            res += x;
            x += 1;
            option = Some(x)
        }else{
            break;
        }
    }
    println!("{}", res);
}
