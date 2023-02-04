use clap::Parser;

#[derive(Debug, clap::Parser)]
struct CommandLineOptions {
    /// The amount of optimization to perform on the code;
    /// 0 = no optimizations,
    /// 1 = some optimizations,
    /// 2 = most optimizations,
    /// 3 = aggressive optimizations
    #[clap(short = 'O', default_value = "2")]
    optimization_level: i32,
    #[clap(short, long = "output")]
    output_file: String,

    input_file: String,
}

fn main() {
    let options = CommandLineOptions::parse();
    println!("{:?}", options)
}
