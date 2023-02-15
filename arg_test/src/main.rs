extern crate getopts;
use gestalt_ratio;
use getopts::Options;
use std::env;
// fn do_work(inp: &str, out: Option<String>) {
//     println!("{}", inp);
//     match out {
//         Some(x) => println!("{}", x),
//         None => println!("No Output"),
//     }
// }

// fn print_usage(program: &str, opts: Options) {
//     let brief = format!("Usage: {} FILE [options]", program);
//     print!("{}", opts.usage(&brief));
// }
extern crate strsim;

use strsim::{
    damerau_levenshtein, hamming, jaro, jaro_winkler, levenshtein, normalized_damerau_levenshtein,
    normalized_levenshtein, osa_distance, sorensen_dice,
};
fn main() {
    let origin_str = "test";
    let target_str = "IamTeaasdfefe";
    let perfect_str = "test";
    let uppercase_str = "TEST";
    let one_char_match = "task";

    println!(
        "{} VS {} :: {:?}",
        origin_str,
        target_str,
        levenshtein(origin_str, target_str)
    );
    println!(
        "{} VS {} :: {:?}",
        origin_str,
        perfect_str,
        levenshtein(origin_str, perfect_str)
    );
    println!(
        "{} VS {} :: {:?}",
        origin_str,
        uppercase_str,
        levenshtein(origin_str, uppercase_str)
    );
    println!(
        "{} VS {} :: {:?}",
        origin_str,
        one_char_match,
        levenshtein(origin_str, one_char_match)
    );
    // command params sample code
    // let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();

    // let mut opts = Options::new();
    // opts.optopt("o", "", "set output file name", "NAME");
    // opts.optflag("h", "help", "print this help menu");
    // let matches = match opts.parse(&args[1..]) {
    //     Ok(m) => m,
    //     Err(f) => {
    //         panic!("{}", f.to_string())
    //     }
    // };
    // if matches.opt_present("h") {
    //     print_usage(&program, opts);
    //     return;
    // }
    // let output = matches.opt_str("o");
    // let input = if !matches.free.is_empty() {
    //     matches.free[0].clone()
    // } else {
    //     print_usage(&program, opts);
    //     return;
    // };
    // do_work(&input, output);
}
