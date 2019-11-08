extern crate clap;

use clap::{Arg, App};

mod travesty;

const DEFAULT_ARRAYSIZE: usize = 3000;
const DEFAULT_MAXPAT: usize = 9;
const DEFAULT_OUTCHARS: usize = 2000;

#[derive(Debug)]
struct TravestyArgumentParser {
    array_size: usize,
    pattern_length: usize,
    out_chars: usize,
    use_verse: bool,
    debug: bool,
    input_file: String,
}

impl TravestyArgumentParser {
    fn parse() -> Self {
        let matches = App::new("Travesty")
            .version("0.1.0")
            .author("Rodney Shupe <rodney@shupe.ca>")
            .about("Teaches argument parsing")
            .arg(Arg::with_name("pattern_length")
                     .short("p")
                     .long("patlen")
                     .takes_value(true)
                     .help("Pattern Length"))
            .arg(Arg::with_name("array_size")
                     .short("a")
                     .long("arrsize")
                     .takes_value(true)
                     .help("ArraySize"))
            .arg(Arg::with_name("out_chars")
                     .short("o")
                     .long("outputsize")
                     .takes_value(true)
                     .help("Number of characters to output."))
            .arg(Arg::with_name("debug")
                     .short("d")
                     .long("debug")
                     .multiple(true)
                     .help("Number of characters to output."))
            .arg(Arg::with_name("use_verse")
                     .short("v")
                     .long("verse")
                     .help("Sets output to verse mode, defaults to prose"))
            .arg(Arg::with_name("INPUT")
                     .help("Sets the input file to use")
                     .required(false)
                     .index(1))
            .get_matches();

        let array_size = matches.value_of("array_size").unwrap_or(&DEFAULT_ARRAYSIZE.to_string()).parse::<usize>().unwrap();

        let mut pattern_length = matches.value_of("pattern_length").unwrap_or(&DEFAULT_MAXPAT.to_string()).parse::<usize>().unwrap();
        if pattern_length > DEFAULT_MAXPAT {
            println!("WARN: patlen ({}) is greater than maximum allowed ({})", pattern_length, DEFAULT_MAXPAT);
            pattern_length = DEFAULT_MAXPAT;
        };

        let out_chars = matches.value_of("out_chars").unwrap_or(&DEFAULT_OUTCHARS.to_string()).parse::<usize>().unwrap();

        let use_verse = matches.is_present("use_verse");
        
        let debug = matches.is_present("debug");

        let input_file = matches.value_of("INPUT").unwrap_or(&"".to_string()).to_string();

        Self { array_size, pattern_length, out_chars, use_verse, debug, input_file }
    }
}

fn main() {
    let params: TravestyArgumentParser = TravestyArgumentParser::parse();
    let mut travesty: travesty::Travesty = travesty::Travesty::init(params.array_size,
        params.pattern_length,
        DEFAULT_MAXPAT,
        params.out_chars,
        params.use_verse,
        params.debug,
        params.input_file);
    travesty.execute();
}
