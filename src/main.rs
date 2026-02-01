// Working on example over here: https://docs.rs/notify/latest/notify/
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mspc};

fn main() -> Result<()> {
    let (tx, rx) = mspc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;
}
