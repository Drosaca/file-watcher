mod args;
mod watcher;
use watcher::FileWatcher;


use args::Options;
fn watch(watcher :&mut FileWatcher, entry :&str) {
    if let Err(e) = watcher.watch(entry) {
        eprintln!("Error: [{entry}] {e}")
    }
}

fn read_events(watcher :&mut FileWatcher) {
    loop {
        match watcher.recv() {
            Ok(event) => {
                match event.kind {  
                    notify::event::EventKind::Create(_) => {
                        println!("CREATE: {}", event.paths[0].to_str().unwrap_or(""))
                    },notify::event::EventKind::Modify(_) => {
                        println!("MODIFY: {}", event.paths[0].to_str().unwrap_or(""))
                    },notify::event::EventKind::Remove(_) => {
                        println!("REMOVE: {}", event.paths[0].to_str().unwrap_or(""))
                    },
                    _ => {},
                }
            }
            Err(e) => {println!("DONE: watcher lost {e}")}
        }
    } 
}

fn main() {
    let options = Options::load();
    let mut watcher = FileWatcher::new().expect("Error: Fatal fail to create watcher");
    for entry in options.files {
        watch(&mut watcher, &entry);
    }
    read_events(&mut watcher)
}