mod craft;
mod cross;
mod garland;
mod macros;
mod task;
mod ui;

use pretty_env_logger;
#[macro_use] use log;
use crate::craft::craft_items;
use crate::task::{Jobs, Task};
use std::path::PathBuf;
use std::ptr::null_mut;
use structopt::StructOpt;
#[macro_use(failure)]
use failure::{Error};

#[derive(StructOpt, Debug)]
#[structopt(name = "Talan")]
struct Opt {
    /// For recipes which have multiple search results this offset is used to
    /// determine the specific recipe to use. Offsets start at 0 for the first
    /// recipe in search results and increment by one for each recipe down.
    #[structopt(short = "i", default_value = "0")]
    recipe_index: u64,

    /// Path to the file containing the XIV macros to use
    #[structopt(name = "macro file", parse(from_os_str))]
    macro_file: PathBuf,

    /// Name of the item to craft
    #[structopt(name = "item name")]
    item_name: String,

    /// Number of items to craft
    #[structopt(short = "c", default_value = "1")]
    count: u64,

    /// Increase delay between actions and UI navigation. Recommended with higher
    /// latency or input lag. [UNIMPLEMENTED]
    #[structopt(short = "d")]
    use_delay: bool,

    /// Gearset to use for this crafting task.
    #[structopt(short = "g", default_value="0")]
    gearset: u64,

    /// Item(s) will be crafted as collectable
    #[structopt(long = "collectable")]
    collectable: bool,
}

fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    let mut window: ui::WinHandle = null_mut();
    // Can this becme map err?
    if !ui::get_window(&mut window) {
        return Err(failure::format_err!("Could not find FFXIV window. Is the client running?"));
    }

    // Grab and parse the config file. Errors are all especially fatal so
    // let them bubble up if they occur.
    let macro_contents =
        macros::parse_file(opt.macro_file).map_err(|e| format!("error parsing macro: `{}`", e));

    let item = garland::fetch_item_info(&opt.item_name)?;
    log::info!("crafting {}", item);

    let tasks = vec![Task {
        item_name: opt.item_name,
        index: opt.recipe_index,
        count: opt.count,
        collectable: opt.collectable,
        actions: macro_contents.unwrap(),
        gearset: opt.gearset,
        job: Jobs::CUL,
    }];
    log::debug!("{:?}", tasks);
    craft_items(window, &tasks);
    Ok(())
}
