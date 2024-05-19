mod datamanip;
use std::io;

mod paperstruct;
use paperstruct::Paper;

fn main() {
    
    // USER INPUT //
    let mut input = String::new();
    let _n = io::stdin().read_line(&mut input);

    //datamanip::testing(&mut input);
    let list = datamanip::tokenizer(&mut input);
    println!("{:?}", list);

    // PAPER STRUCT //
    let tags: Vec<String> = list[2..].to_vec();
    let paper = Paper::new(&list[0], &list[1], tags);
    println!("{:#?}", paper);
}
