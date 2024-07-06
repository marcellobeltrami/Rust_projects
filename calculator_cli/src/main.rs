use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

//Structure of your CLI interface
struct Args{
    #[arg(short = 's', long)]
    operation:String,
    
    #[arg(short= 'n', long)]
    num1:f64,
    
    #[arg(short= 'N', long)]
    num2:f64
}


fn main() {
    //Parses arguments
    let args = Args::parse();

    // Uses arguments to carry calculation
    let result:f64 = match args.operation.as_str() {
        "+" => args.num1 + args.num2,
        "-" => args.num1 - args.num2,
        "/" => args.num1 / args.num2,
        "x" => args.num1 * args.num2,
        _ => panic!("operation not supported")
        
    };

    println!("Result {}", result);

}
