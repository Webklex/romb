mod scanner;

use clap::{App, Arg};
use std::time::Duration;
use crate::scanner::ScannerError;

fn main() {
    let arguments = App::new("romb")
        .version("0.0.1")
        .about("Check many ports at once")
        .arg(
            Arg::new("target")
                .help("The target to scan")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("timeout")
                .help("Connection timeout")
                .long("timeout")
                .short("t".parse().unwrap())
                .default_value("10"),
        )
        .arg(
            Arg::new("start_port")
                .help("Lowest port")
                .long("start_port")
                .short("s".parse().unwrap())
                .default_value("1"),
        )
        .arg(
            Arg::new("max_port")
                .help("Highest port")
                .long("max_port")
                .short("m".parse().unwrap())
                .default_value("90"),
        )
        .arg(
            Arg::new("response")
                .help("Check if the sockets runs any bytes")
                .long("response")
                .short("r".parse().unwrap()),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let target = arguments.value_of("target").unwrap();
    let timeout = arguments
        .value_of("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(10);
    let start_port = arguments
        .value_of("start_port")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(1);
    let max_port = arguments
        .value_of("max_port")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(start_port + 1);

    let mut opt = scanner::build_options();
    *opt.udp_mut() = false;
    *opt.tcp_mut() = false;
    *opt.response_mut() = false;

    //let opt = scanner::build_options();
    let mut s = scanner::build_scanner(opt);
    s.set_target(target.to_string());
    s.set_port_range(start_port, max_port);
    s.set_timeout(Duration::from_secs(timeout));

    match s.start() {
        Ok(()) => (),
        Err(e) => {
            match e {
                ScannerError::InvalidPortRange => println!("Scanner error: {}", e),
                ScannerError::InvalidTarget => println!("Scanner error: {}", e),
            }
        }
    };
}