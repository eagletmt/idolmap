extern crate clap;
extern crate env_logger;
extern crate idolmap;

fn main() {
    env_logger::init().expect("Failed to initialize env_logger");

    let matches = clap::App::new("idolmap")
        .version("0.1.0")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(
            clap::SubCommand::with_name("aikatsu")
                .setting(clap::AppSettings::SubcommandRequired)
                .subcommand(clap::SubCommand::with_name("update").about(
                    "Update aikatsu CSV files",
                )),
        )
        .subcommand(
            clap::SubCommand::with_name("lovelive")
                .setting(clap::AppSettings::SubcommandRequired)
                .subcommand(clap::SubCommand::with_name("update").about(
                    "Update lovelive CSV files",
                )),
        )
        .subcommand(
            clap::SubCommand::with_name("pripara")
                .setting(clap::AppSettings::SubcommandRequired)
                .subcommand(clap::SubCommand::with_name("update").about(
                    "Update pripara CSV files",
                )),
        )
        .get_matches();

    match matches.subcommand() {
        ("aikatsu", Some(matches)) => aikatsu(matches),
        ("lovelive", Some(matches)) => lovelive(matches),
        ("pripara", Some(matches)) => pripara(matches),
        _ => unreachable!(),
    }
}

fn aikatsu<'a>(matches: &clap::ArgMatches<'a>) {
    match matches.subcommand() {
        ("update", _) => idolmap::aikatsu::update_all(),
        _ => unreachable!(),
    }
}

fn lovelive<'a>(matches: &clap::ArgMatches<'a>) {
    match matches.subcommand() {
        ("update", _) => idolmap::lovelive::update_all(),
        _ => unreachable!(),
    }
}

fn pripara<'a>(matches: &clap::ArgMatches<'a>) {
    match matches.subcommand() {
        ("update", _) => idolmap::pripara::update_all(),
        _ => unreachable!(),
    }
}
