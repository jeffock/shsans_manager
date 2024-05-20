mod datamanip;
use std::{fs::OpenOptions, io::{self, Seek, Write, BufReader}};

mod paperstruct;
use paperstruct::Paper;

use serde_json::{self, from_reader, to_writer_pretty};

fn main() -> std::io::Result<()>{
    
    // USER INPUT //
    let mut input = String::new();
    let _n = io::stdin().read_line(&mut input);

    //datamanip::testing(&mut input);
    let list = datamanip::tokenizer(&mut input);
    println!("{:?}", list);

    // PAPER STRUCT //
    let doi: String = list[0].to_string();
    let title: String = list[1].to_string();
    let tags: Vec<String> = list[2..].to_vec();
    let paper = Paper::new(doi, title, tags);
    println!("{:#?}", paper);

    // FILE creation/writing //
    let json_path = "papers.json";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(json_path)?;

    // vector of papers //
    let mut papers: Vec<Paper> = match from_reader(BufReader::new(&file)) {
        Ok(papers) => papers,
        Err(_) => Vec::new(),
    };

    papers.push(paper);
    // rewind file //
    file.set_len(0)?; // clear contents
    file.seek(std::io::SeekFrom::Start(0))?; // rewind file pointer
    // write updated vector //
    to_writer_pretty(&file, &papers)?;

    Ok(())
}

