//use std::collections::HashMap;
mod xiv_macro;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Item(s) being crafted are collectible
    #[structopt(short = "c", long = "collectible")]
    collectible: bool,
    /// Print verbose information during execution
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    /// Number of items to craft
    #[structopt(short = "n", long = "count", default_value = "1")]
    count: u32,
    /// Path to the file containing the XIV macro to use
    #[structopt(name = "macro file", parse(from_os_str))]
    macro_file: PathBuf,
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();

    println!("{:?}", opt);
    let entries = xiv_macro::parse_file(opt.macro_file);
    for i in &entries {
        println!("{}", i);
    }

    //let _recipe = recipe::Recipe {};
    //let settings_file = &args[1];
    //let _macro_file = &args[2];
    //let _run_count = &args[3];
    //let mut settings = config::Config::default();
    //settings.merge(config::File::with_name(settings_file))?;
    return Ok(());
}
