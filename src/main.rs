// Working on example over here: https://docs.rs/notify/latest/notify/
use notify::{Event, RecursiveMode, Result, Watcher};
use notify::event;
use std::{path::Path, sync::mpsc, fs::copy, path::PathBuf};


const BACKUP_DIR : &str = "/home/sivabalansm/sc/watchback/test/backup/";

fn copy_to_backup(file : &PathBuf) {
    let file_name = file.file_name().unwrap().to_str().unwrap();
    let to_dir = format!("{}{}", BACKUP_DIR, file_name);
    copy(file, to_dir).unwrap();
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
