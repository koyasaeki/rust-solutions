use clap::{value_parser, Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Koya Saeki <koyasaeki@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not output print newline")
                .num_args(0),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|x| x.to_string())
        .collect();
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
