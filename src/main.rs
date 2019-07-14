extern crate clap;
extern crate image;
extern crate rayon;
extern crate sheep;
extern crate walkdir;

mod cli;
mod pack;
mod strip;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("pack", args) => handle_pack(args.unwrap().value_of("input"), args.unwrap().value_of("output")),
        ("strip", args) => handle_strip(args.unwrap().value_of("input")),
        ("completions", _) => handle_completions(),
        _ => {}
    }
}

// Completions dont appear to work
fn handle_completions() {
    cli::build_cli().gen_completions_to("asset_manager", clap::Shell::Zsh, &mut std::io::stdout());
}

fn handle_pack(input: Option<&str>, output: Option<&str>) {
    pack::pack_tiles(input.unwrap(), output.unwrap())
}

fn handle_strip(possible_path: Option<&str>) {
    strip::strip_transparency(possible_path.unwrap())
}
