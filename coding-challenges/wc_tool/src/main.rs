use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>
}
#[derive(Subcommand)]
enum Commands{
    /// Count the number of bytes in the text
   #[command(about = "Count the number of bytes in a file", short_flag = 'c')] 
    Count,
    /// count the number of lines in the text
  #[command(about = "Count the number of lines in a file", short_flag = 'l')]
    Line,
  #[command(about ="Count the number of words in the file", short_flag = 'w')]
    Words,
  #[command(about = "Count the number of characters in the file", short_flag = 'm')]
    Characters,
}


fn count_bytes_in_file(filename: String) -> io::Result<usize> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer.len())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn count_lines(filename:String)->anyhow::Result<usize>{
    let mut count_lines= 0;
    
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|_|{
                count_lines += 1;
        })
    }
    Ok(count_lines)
}

async fn count_words(filename:String)->anyhow::Result<usize>{
    let mut count_words= 0;
    if let Ok(words) = read_lines(filename){
        words.for_each(|word|{
            if let Ok(w) = word{
                if !w.trim().is_empty(){
                let collection: Vec<&str> = w.split_whitespace().collect();
                count_words += collection.len();
                }
            }
        });
       
    };
    Ok(count_words)
}
async fn count_characters(filename:String)->anyhow::Result<usize>{
   let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    
    let number_of_characters = content.chars().count();
    
    Ok(number_of_characters)
}
#[tokio::main]
async fn main()-> anyhow::Result<()>{
    let cli = Args::parse();
    
    match cli.command {
        Some(Commands::Count) => {
            let count=count_bytes_in_file("Test.txt".to_string())?;
            println!("{count}");
        },
        Some(Commands::Line)=> {
            let lines_number = count_lines("Test.txt".to_string()).await?;
            println!("{lines_number}");
        },
        Some(Commands::Words)=> {
            let words_number = count_words("Test.txt".to_string()).await?;
            println!("{words_number}");
        },
        Some(Commands::Characters)=>{
            let characters_number = count_characters("Test.txt".to_string()).await?;
            println!("{characters_number}")
        },
        None =>println!("Run with --help to see all the commands")
    
    };
    Ok(())
}
