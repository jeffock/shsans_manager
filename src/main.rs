mod datamanip;
use std::io;

fn main() {
    
    // USER INPUT //
    let mut input = String::new();
    let _n = io::stdin().read_line(&mut input);

    //datamanip::testing(&mut input);
    let list = datamanip::tokenizer(&mut input);
    println!("{:?}", list);
}
