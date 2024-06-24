use clap::{Command, ArgMatches, Arg};


fn main() {

    /* 
    
    cli:

    "--log-file", metavar="L", default="{ROOTDIR}/.nmk/nmk.log", help="write logs to L (default: {ROOTDIR}/.nmk/nmk.log)"
    
    
    
     */


    let matches: ArgMatches = clap::Command::new("onmk")
    .about("Next-gen make-like build system")
    .version("0.1")
    .long_about("This is a long about ... ... ..") /* written whe using --help */
    .arg(
        Arg::new("log_file")
        .long("log-file")
        .value_name("L")
        .help("write logs to L (default: {ROOTDIR}/.nmk/nmk.log)")
    )
    .get_matches();
    println!("Matches: {:#?}", matches);




}
