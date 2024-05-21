mod datamanip;
use std::{fs::OpenOptions, io::{self, Seek, BufReader}};

mod paperstruct;
use paperstruct::Paper;

mod apifunc;

use serde_json::{self, from_reader, to_writer_pretty};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // USER INPUT //
    let mut input = String::new();
    let _n = io::stdin().read_line(&mut input);

    //datamanip::testing(&mut input);
    let list = datamanip::tokenizer(&mut input);
    println!("{:?}", list);

    // PAPER STRUCT //
    let mut paper: Paper = match popul_paper(&list).await {
        Ok(paper) => paper,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e);
        }
    };
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

async fn popul_paper(list: &[String]) -> Result<Paper, Box<dyn std::error::Error>> {
    if list.len() == 1 {
        apifunc::fetch_paper(&list[0]).await
    } else {
        if list.len() >= 3 {
            let doi = list[0].clone();
            let title = list[1].clone();
            let tags = list[2..].to_vec();
            Ok(Paper::new(doi, title, tags))
        } else {
            Err("List does not contain enough elements".into())
        }
    }
}

