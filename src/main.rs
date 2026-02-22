// Working on example over here: https://docs.rs/notify/latest/notify/
use notify::{Event, RecursiveMode, Result, Watcher};
use notify::event;
use std::{path::Path, sync::mpsc};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("event kind: {:?}", event.kind);
                // println!("event: {:?}", event);
                match event.kind {
                    event::EventKind::Create(event::CreateKind::File) => println!("new file created!"),
                    _ => println!("Not implemented")
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
