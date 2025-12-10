use notify::{Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::{sync::mpsc, fs};
use std::sync::mpsc::Receiver;

pub struct FileWatcher {
    rx: Receiver<Result<Event>>,
    watcher: RecommendedWatcher
}

impl FileWatcher {
    pub fn new()-> Result<Self> {
        let (tx, rx) = mpsc::channel::<Result<Event>>();
        let watcher = notify::recommended_watcher(tx)?;
        Ok(Self{ rx, watcher})
    }
    pub fn watch(&mut self, entry :&str) -> Result<()> {
        let path = fs::canonicalize(entry)?;
        println!("WATCHING: {}", path.to_str().unwrap_or(""));
        self.watcher.watch(path.as_path() ,RecursiveMode::Recursive)
    }
    pub fn recv(&self) -> Result<Event> {
        match self.rx.recv() {
            Ok(event) => {event}
            Err(e) => Err(e.into())
        }
    }
}