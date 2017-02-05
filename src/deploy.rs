use clap::ArgMatches;
use config::Config;

pub fn run(matches: &ArgMatches, config: &Config) {
    let stage = matches.value_of("stage").expect("Expected required stage");

    println!("{:?}", config);
    println!("{}", stage);
}
