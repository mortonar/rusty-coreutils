use chrono::offset::Utc;
use chrono::DateTime;
use clap::Parser;
use std::fs;
use std::fs::DirEntry;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    files: Vec<String>,
    /// do not ignore entries starting with "."
    #[clap(short, long)]
    all: bool,
    /// use a long listing format
    #[clap(short)]
    long: bool,
}

fn main() {
    let mut cli = Cli::parse();
    if cli.files.is_empty() {
        cli.files.push(String::from("./"));
    }
    for f in cli.files {
        let paths = fs::read_dir(f).unwrap();
        let mut entries: Vec<DirEntry> = paths.map(|p| p.unwrap()).collect();

        // TODO current and parent directory are not listed when "-a" is given
        if !cli.all {
            entries.retain(|p| !p.file_name().into_string().unwrap().starts_with('.'));
        }

        if cli.long {
            entries.iter().for_each(|e| {
                let metadata = e.metadata().unwrap();
                let datetime: DateTime<Utc> = metadata.modified().unwrap().into();
                // TODO align the columns on this
                println!(
                    "{} {} {} {} {} {} {}",
                    unix_mode::to_string(metadata.permissions().mode()),
                    metadata.nlink(),
                    users::get_user_by_uid(metadata.uid())
                        .unwrap()
                        .name()
                        .to_str()
                        .unwrap(),
                    users::get_group_by_gid(metadata.gid())
                        .unwrap()
                        .name()
                        .to_str()
                        .unwrap(),
                    metadata.size(),
                    datetime.format("%h %d %R"),
                    e.file_name().into_string().unwrap()
                )
            });
        } else {
            let entries: Vec<String> = entries
                .iter()
                .map(|e| e.file_name().into_string().unwrap())
                .collect();
            println!("{}", entries.join(" "));
        }
    }
}
