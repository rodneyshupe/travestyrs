extern crate clap;

use clap::{Arg, App};

mod travesty;

const DEFAULT_ARRAYSIZE: usize = 3000;
const DEFAULT_PATTERN_LENGTH: usize = 9;
const DEFAULT_PATTERN_LENGTH_MAX: usize = 15;
const DEFAULT_PATTERN_LENGTH_MIN: usize = 3;
const DEFAULT_OUTCHARS: usize = 2000;
const DEFAULT_LINE_WIDTH: usize = 50;


#[derive(Debug)]
struct TravestyArgumentParser {
    buffer_size: usize,
    pattern_length: usize,
    out_chars: usize,
    line_width: usize,
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
                     .long("pattern-length")
                     .takes_value(true)
                     .help("Pattern Length"))
            .arg(Arg::with_name("buffer_size")
                     .short("b")
                     .long("buffer-size")
                     .takes_value(true)
                     .help("The size of the buffer to be analyzed. The larger this is the slower the output will appear"))
            .arg(Arg::with_name("out_chars")
                     .short("o")
                     .long("output-size")
                     .takes_value(true)
                     .help("Number of characters to output"))
            .arg(Arg::with_name("line_width")
                     .short("l")
                     .long("line-width")
                     .takes_value(true)
                     .help("Approximate line length of the output"))
            .arg(Arg::with_name("debug")
                     .short("d")
                     .long("debug")
                     .multiple(true)
                     .help("Number of characters to output."))
            .arg(Arg::with_name("use_verse")
                     .long("verse")
                     .help("Sets output to verse mode, defaults to prose"))
            .arg(Arg::with_name("INPUT")
                     .help("Sets the input file to use")
                     .required(false)
                     .index(1))
            .get_matches();

        let buffer_size = matches.value_of("buffer_size").unwrap_or(&DEFAULT_ARRAYSIZE.to_string()).parse::<usize>().unwrap();

        let mut pattern_length = matches.value_of("pattern_length").unwrap_or(&DEFAULT_PATTERN_LENGTH.to_string()).parse::<usize>().unwrap();
        if pattern_length > DEFAULT_PATTERN_LENGTH_MAX {
            println!("WARN: pattern-length ({}) is greater than maximum allowed ({})", pattern_length, DEFAULT_PATTERN_LENGTH_MAX);
            pattern_length = DEFAULT_PATTERN_LENGTH_MAX;
        } else if pattern_length < DEFAULT_PATTERN_LENGTH_MIN {
            println!("WARN: pattern-length ({}) is less than minimum allowed ({})", pattern_length, DEFAULT_PATTERN_LENGTH_MIN);
            pattern_length = DEFAULT_PATTERN_LENGTH_MIN;
        }

        let out_chars = matches.value_of("out_chars").unwrap_or(&DEFAULT_OUTCHARS.to_string()).parse::<usize>().unwrap();

        let line_width = matches.value_of("line_width").unwrap_or(&DEFAULT_LINE_WIDTH.to_string()).parse::<usize>().unwrap();

        let use_verse = matches.is_present("use_verse");

        let debug = matches.is_present("debug");

        let input_file = matches.value_of("INPUT").unwrap_or(&"".to_string()).to_string();

        Self { buffer_size, pattern_length, out_chars, use_verse, line_width, debug, input_file }
    }
}

fn main() {
    let params: TravestyArgumentParser = TravestyArgumentParser::parse();
    let mut travesty: travesty::Travesty = travesty::Travesty::init(params.buffer_size,
        params.pattern_length,
        DEFAULT_PATTERN_LENGTH_MAX,
        params.out_chars,
        params.line_width,
        params.use_verse,
        params.debug,
        params.input_file);
    travesty.execute();
}
