use std::path::PathBuf;

use structopt::StructOpt;

use rocket_contrib::serve::StaticFiles;

#[derive(Debug, StructOpt)]
struct Options
{
    #[structopt(parse(from_os_str))]
    root: PathBuf
}

#[paw::main]
fn main(args: Options) {
    rocket::ignite()
        .mount("/", StaticFiles::from(args.root))
        .launch();
}
