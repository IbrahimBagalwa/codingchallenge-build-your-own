use std::fs::File;
use std::io::{self, BufReader, Read};
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

#[tokio::main]
async fn main()-> anyhow::Result<()>{
    let cli = Args::parse();
    
    
    match cli.command {
        Some(Commands::Count) => {
            let count=count_bytes_in_file("Test.txt".to_string())?;
            println!("{count}");
        },
        None =>println!("Run with --help to see all the commands")
    
    };
    Ok(())
}
