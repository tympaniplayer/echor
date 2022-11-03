use clap::{Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Nate Palmer <nate@natepalmer.dev")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input Text")
                .num_args(1..)
                .required(true)

        )
        .arg(
            Arg::new("omit_newline")
            .short('n')
            .help("Do not print newline")
            .action(ArgAction::SetTrue)
        )
        .get_matches();

        println!("{:?}", matches);
}
