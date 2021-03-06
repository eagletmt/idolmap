fn main() {
    env_logger::init();

    let matches = clap::App::new("idolmap")
        .version("0.1.0")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(
            clap::SubCommand::with_name("aikatsu")
                .setting(clap::AppSettings::SubcommandRequired)
                .subcommand(
                    clap::SubCommand::with_name("update").about("Update aikatsu CSV files"),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("lovelive")
                .setting(clap::AppSettings::SubcommandRequired)
                .subcommand(
                    clap::SubCommand::with_name("update").about("Update lovelive CSV files"),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("prichan")
                .setting(clap::AppSettings::SubcommandRequired)
                .subcommand(
                    clap::SubCommand::with_name("update").about("Update prichan CSV files"),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("csv")
                .setting(clap::AppSettings::SubcommandRequired)
                .subcommand(
                    clap::SubCommand::with_name("bundle")
                        .about("Bundle CSV files for uploading to Google Maps")
                        .arg(clap::Arg::with_name("FILE").required(true).multiple(true)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("aikatsu", Some(matches)) => aikatsu(matches),
        ("lovelive", Some(matches)) => lovelive(matches),
        ("prichan", Some(matches)) => prichan(matches),
        ("csv", Some(matches)) => csv(matches),
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

fn prichan<'a>(matches: &clap::ArgMatches<'a>) {
    match matches.subcommand() {
        ("update", _) => idolmap::prichan::update_all(),
        _ => unreachable!(),
    }
}

fn csv<'a>(matches: &clap::ArgMatches<'a>) {
    match matches.subcommand() {
        ("bundle", Some(matches)) => idolmap::csv::bundle(matches.values_of("FILE").unwrap()),
        _ => unreachable!(),
    }
}
