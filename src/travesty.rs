extern crate regex;
extern crate rand;

use std::io::{self, Read};
use std::fs;
use regex::Regex;
use rand::Rng;

const ARRAYSIZE_MAX: usize = 10000;
const ASCII_SPACE: u8 = 32;
const ASCII_DEL: u8 = 127;

pub struct Travesty {
    buffer_size: usize,
    max_pattern_length: usize,
    pattern_length: usize,
    use_verse: bool,
    input_file: String,
    buffer: String,
    buffer_array: String,
    freq_array: [usize; 256],
    start_skip: [usize; 256],
    skip_array: [usize; ARRAYSIZE_MAX],
    pattern: String,
    out_chars: usize,
    line_width: usize,
    char_count: usize,
    near_end: bool,
    debug: bool,
}

impl Travesty {
    pub fn init(buffer_size_param: usize,
            pattern_length_param: usize,
            max_pattern_length_param: usize,
            out_chars_param: usize,
            line_width_param: usize,
            use_verse_param: bool,
            debug_param: bool,
            input_file_param: String ) -> Self {

        let mut buffer_size = buffer_size_param;
        if buffer_size > ARRAYSIZE_MAX { buffer_size = ARRAYSIZE_MAX }
        let max_pattern_length = max_pattern_length_param;
        let pattern_length = pattern_length_param;
        let use_verse = use_verse_param;
        let debug = debug_param;
        let input_file = input_file_param;
        let out_chars = out_chars_param;
        let line_width = line_width_param;

        let buffer = String::new();
        let buffer_array = String::new();
        let freq_array: [usize; 256] = [0; 256];
        let start_skip: [usize; 256] = [0; 256];
        let skip_array: [usize; ARRAYSIZE_MAX] = [0; ARRAYSIZE_MAX];
        let pattern = String::new();
        let char_count = 0;
        let near_end = false;

        Self { buffer_size, max_pattern_length, pattern_length, use_verse,
                input_file, buffer, buffer_array, freq_array, start_skip,
                skip_array, pattern, out_chars, line_width, char_count, near_end, debug }
    }

    // FreqArray is indexed by 93 probable ASCII characters, from ASCII_SPACE to ASCII_DEL.
    // Its elements are all set to zero.
    fn clear_freq_array(&mut self) {
        for ch in ASCII_SPACE..ASCII_DEL {
            self.freq_array[ch as usize] = 0
        }
    }

    // Reads input_file from disk into buffer_array, cleaning it up and reducing any run of
    // whitespace to a single space.  (If no inputfile is supplied stdin is used instead)
    // Once read it then copies to end of array a string of its opening characters as long
    // as the pattern_length, in effect wrapping the end to the beginning.
    fn fill_array(&mut self) {
        if self.input_file.trim().is_empty() {
            io::stdin().read_to_string(&mut self.buffer).expect("ERROR: Something went wrong reading from stdin");;
        } else {
            self.buffer = fs::read_to_string(&mut self.input_file).expect("ERROR: Something went wrong reading the file");
        }
        self.buffer = self.buffer.trim().to_string();

        let re = Regex::new(r"(\s{2,}|\n)").unwrap();
        let buffer_array_tmp = &re.replace_all(&self.buffer, " ");
        self.buffer_array = buffer_array_tmp[0..self.buffer_size-(self.max_pattern_length + 1)].to_string();
        self.buffer_array.push_str(&" ".to_string());
        self.buffer_array.push_str(&buffer_array_tmp[0..self.pattern_length].to_string());

        println!("Characters read, plus wraparound = {}", self.buffer_array.chars().count());
    }

    //  User selects "order" of operation, an integer, n, in the range 1 .. 9. The input
    //  text will henceforth be scanned in n-sized chunks. The first n-1 characters of the
    //  input file are placed in the "Pattern" Array. The Pattern is written at the head of output.
    fn first_pattern(&mut self) {
        self.pattern = self.buffer_array[0..self.pattern_length].to_string();
        self.char_count = self.pattern_length;
        self.near_end = false;
        if self.use_verse { print!(" ") } // Align first line
        print!("{}", self.pattern);
    }

    // The i-th entry of skip_array contains the smallest index j < i such that
    // buffer_array[O] = buffer_array[i]. Thus skip_array links together all identical characters
    // in buffer_array.  start_skip contains the index of the first occurrence of each
    // character, These two arrays are used to skip the matching routine through the
    // text, stopping only at locations whose character matches the first character
    // in Pattern.
    fn init_skip_array(&mut self) {
        for ch in ASCII_SPACE..ASCII_DEL {
            self.start_skip[ch as usize] = self.buffer_array.as_bytes().len();
        }
        for j in (1..self.buffer_array.as_bytes().len()).rev() {
            let ch = self.buffer_array.as_bytes()[j - 1];
            self.skip_array[j - 1] = self.start_skip[ch as usize];
            self.start_skip[ch as usize] = j;
        }
    }

    // Checks buffer_array for strings that match Pattern; for each match found, notes
    // following character and increments its count in FreqArray. Position for first
    // trial comes from StartSkip; thereafter positions are taken from SkipArray.
    // Thus no sequence is checked unless its first character is already known to
    // match first character of Pattern.
    fn match_pattern(&mut self) {
        let ch = self.pattern.as_bytes()[0];
        let mut i: usize = self.start_skip[ch as usize] - 1;        // i is 1 to left of the Match start
        while i <= self.buffer_array.chars().count() - self.pattern_length - 1 {
            if self.buffer_array[i..i+self.pattern_length] == self.pattern {
                let next_char = self.buffer_array.as_bytes()[i + self.pattern_length];
                self.freq_array[next_char as usize] += 1;
            }
            i = self.skip_array[i] - 1
        }
    }

    // It is chosen at Random from characters accumulated in FreqArray during
    //last scan of input.
    fn get_next_char(&mut self) -> char {
        let mut total = 0;
        for ch in ASCII_SPACE..ASCII_DEL {
            total = total + self.freq_array[ch as usize]; // Sum counts in FreqArray
        }
        let mut toss = rand::thread_rng().gen_range(1, total + 1);
        let mut counter: u8 = ASCII_SPACE - 1;
        while toss > 0 {
            counter += 1;
            if toss > self.freq_array[counter as usize] {
                toss = toss - self.freq_array[counter as usize];
            } else {
                toss = 0;
            }
        }
        counter as char
    }

    // The next character is written.  Output lines will
    // average self.line_width characters in length. If "Verse" option has been selected,
    // a new line will commence after any word that ends with "'"in input file.
    // Thereafter lines will be indented until the self.line_width-character average has
    // been made up.
    fn write_character(&mut self, new_char: char) {
        if new_char != (ASCII_DEL as char) {
            print!("{}", new_char);
        }
        self.char_count = self.char_count + 1;
        if self.char_count % self.line_width == 0 { self.near_end = true; }
        if self.use_verse && new_char == (ASCII_DEL as char) { println!(""); }
        if self.near_end && new_char == (ASCII_SPACE as char) {
            println!("");
            if self.use_verse { print!("   ") }
            self.near_end = false;
        }
    }

    // This removes the first character of the Pattern and appends the character
    // just printed. FreqArray is zeroed in preparation for a new scan.
    fn new_pattern(&mut self, next_char: char) {
        self.pattern = self.pattern[1..self.pattern_length].to_string();
        self.pattern.push_str(&next_char.to_string());
        self.clear_freq_array();
    }

    pub fn output_debug_info(&mut self, show_buffer: bool, show_buffer_array: bool) {
        print!("buffer_size={} ", self.buffer_size);
        print!("pattern_length={} ", self.pattern_length);
        print!("out_chars={} ", self.out_chars);
        print!("input_file={} ", self.input_file);
        print!("buffer Size= {} ", self.buffer.chars().count());
        print!("buffer_array Size={} ", self.buffer_array.chars().count());
        println!("\n");
        if show_buffer {
            println!("Buffer Data:");
            println!("{}", self.buffer);
            println!("");
        }
        if show_buffer_array {
            println!("buffer_array:");
            println!("{}", self.buffer_array);
            println!("");
        }
    }

    pub fn execute(&mut self) {
        self.clear_freq_array();
        self.fill_array();

        if self.debug { self.output_debug_info(false, false); }

        self.first_pattern();
        self.init_skip_array();

        let mut next_char: char = ' ';
        while self.char_count < self.out_chars || next_char != ' ' {
            self.match_pattern();
            next_char = self.get_next_char();
            self.write_character(next_char);
            self.new_pattern(next_char);
        }

        println!("");
        println!("");
        println!("Output: {} characters.", self.char_count);
    }
}
