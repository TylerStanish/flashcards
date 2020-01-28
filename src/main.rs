use clap::{App, Arg};

mod repl;

fn main() {
    let matches = App::new("Tarjetas")
        .version("1.0")
        .author("Tyler S. <tystanish@gmail.com>")
        .about("Una CLT para archivar y practicar tu vocabulario")
        .arg(
            Arg::with_name("ayuda")
                .short("h")
                .long("ayuda")
                .value_name("AYUDA")
                .help("Mostrar este mensaje")
                .takes_value(false),
        );
    repl::start();
}
