use clap::Parser;



#[derive(Parser, Debug)]
#[command(name = "file-watcher", long_about = r#"
Monitors given entries events

POSSIBLE EVENTS:
    CREATE, REMOVE, MODIFY
OTHER:
    WATCHING, Error
"#)]
pub struct Options {
    #[arg(short, long)]
    pub files: Vec<String>,

}

impl Options {
    pub fn load() -> Self{
        Self::parse()
    }
}