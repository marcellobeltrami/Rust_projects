use pulldown_cmark::{Parser, Options};
use std::fs::{File,read_to_string,read_dir};
use std::io::{self, Write};
use std::path::PathBuf;
use clap::Parser as CLI;

// V TODO Automate parsing all .md files in a directory and outpuit html file with the same name
// V TODO Add CLI

fn main()->io::Result<()> {
    
    #[derive(CLI, Debug)]
    #[command(version = "1.0", about = "Converts Markdown files to HTML", long_about = None)]

    //Structure of your CLI interface
    struct Args{
        #[arg(short = 'i', long, help = "Specify the input directory")]
        input_dir:String,
        
        #[arg(short= 'o', long, help = "Specify the output directory")]
        output_dir:String
    }

    let args = Args::parse();
    
    let paths = read_dir(PathBuf::from(args.input_dir))?; // ? is used to handle errors. Same as expect(). 

    // Set up options and HTML parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    for file_md in paths
    {
        let file_md = file_md?;
        let out_name = file_md.file_name();
        println!("Converting {:?}...", out_name);
        let markdown_input = read_to_string(file_md.path())?;
        
        
        let markdown_input_str:&str = &markdown_input; // obtains &str reference to String
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(markdown_input_str, options); // Parses the md string using specified options with default  

        // Write to String buffer.
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);

        
        //Extracts file name and prepares output.
        let mut output_path = PathBuf::from(&args.output_dir);
        output_path.push(&out_name);
        output_path.set_extension("html");

        //Write html string to file.
        let mut file_out = File::create(output_path)?; 
        file_out.write_all(html_output.as_bytes())?;
        
        // Updates output line and removes % by printing new line
        print!("\x1B[A");
        print!("\r");
        print!("Converting {:?}...ok!", out_name);
        println!();
    }

    Ok(())

}



