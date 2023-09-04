mod app;
use app::App;
use app::Config;

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "0.1",
    author = "Vincent S.",
    about = "An Interactive JQ tool"
)]
struct Args {
    #[arg(help = "Input json file")]
    input: String,
}




fn main() -> Result<(), std::io::Error> {

    let args = Args::parse();

    let config = Config::new();
    let mut app = App::new(args.input, &config)?;
    app.run()?;
    Ok(())
}
