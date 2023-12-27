mod backup;
mod restore;
mod writer;

use argparser::ArgumentConfig;

pub fn main() {
    let args = ArgumentConfig::init();
    println!("{:#?}", args);

    let mut iter = args.commands.iter();
    match iter.next() {
        None => { writer::usage(0, true); }
        Some(action) => {
            match action.trim() {
                "b" | "backup" => { backup::backup(iter.next(), iter.next()); }
                "r" | "restore" => { restore::restore(iter.next(), iter.next()); }
                &_ => { writer::usage(0, true); }
            }
        }
    }
}
