use std::num::ParseIntError;
use std::process::Command;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

fn parse_hex(src: &str) -> Result<u16, ParseIntError> {
    u16::from_str_radix(src.strip_prefix("0x").unwrap_or(src), 16)
}

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(long, short, parse(try_from_str = parse_hex))]
    vendor_id: u16,

    #[structopt(long, short, parse(try_from_str= parse_hex))]
    product_id: u16,

    /// Check interval [unit: secs]
    #[structopt(long, short, default_value = "1")]
    interval: u64,

    #[structopt(long, short)]
    before: Option<String>,

    #[structopt(long, short)]
    after: Option<String>,

    #[structopt(long, short)]
    debug: bool,
}

fn is_connected(vendor_id: u16, product_id: u16) -> bool {
    rusb::open_device_with_vid_pid(vendor_id, product_id).is_some()
}

fn run_command(cmdline: &str) -> std::io::Result<()> {
    let mut parts = cmdline.split_ascii_whitespace();

    Command::new(parts.next().unwrap())
        .args(&parts.collect::<Vec<_>>())
        .status()
        .map(|_| ())
}

fn main() {
    let options = Options::from_args();
    let interval = Duration::from_secs(options.interval);

    if rusb::open_device_with_vid_pid(options.vendor_id, options.product_id).is_none() {
        println!("unable to find the device.");
        return;
    }

    if let Some(before) = options.before {
        let _ = run_command(&before);
    }

    loop {
        thread::sleep(interval);
        if !is_connected(options.vendor_id, options.product_id) {
            break;
        }

        if options.debug {
            println!("still connected...");
        }
    }

    println!("disconnected...");

    if let Some(after) = options.after {
        let _ = run_command(&after);
    }
}
