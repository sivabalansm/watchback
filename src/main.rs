// Working on example over here: https://docs.rs/notify/latest/notify/
use notify::{Event, RecursiveMode, Result, Watcher};
use notify::event;
use std::{path::Path, sync::mpsc, fs::copy, path::PathBuf};


const BACKUP_DIR : &str = "/home/sivabalansm/sc/watchback/test/backup/";

fn copy_to_backup(file : &PathBuf) -> bool {
    let file_name = file.file_name().expect("Str  plz");
    let file_name = file_name.to_str().expect("Str needed");
    let to_dir = format!("{}{}", BACKUP_DIR, file_name);
    match copy(file, to_dir) {
        Ok(n) => { println!("Copied successfully, return code {:?}", n); return true },
        Err(e) => { println!("Copy error {:?}", e); return false },
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("event kind: {:?}", event.kind);
                match event.kind {
                    event::EventKind::Create(event::CreateKind::File) => {
                        println!("new file created at {:?}!", event.paths);
                        println!("Coping file to backup dir");
                        let path_str : &PathBuf = &event.paths[0];
                        copy_to_backup(path_str);
                    },
                    _ => println!("Not implemented")
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
