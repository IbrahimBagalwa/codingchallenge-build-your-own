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
  #[command(about = "Count the number of lines in a file", short_flag = 'l')]
    Line
}


fn count_bytes_in_file<P>(filename: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
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
            println!("{lines_number}")
        },
        None =>println!("Run with --help to see all the commands")
    
    };
    Ok(())
}
