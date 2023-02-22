#[macro_use]
extern crate log;

mod input;

use input::InputEvent;
use std::process::{exit, Command};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, SeekFrom, Seek};
use std::path::PathBuf;
use std::{env, mem};
use getopts::Options;

fn root_check() {
    let euid = unsafe { libc::geteuid() };
    
    if euid != 0 {
        panic!("Must run as root user");
    }
}

pub fn get_config() -> Config {
    root_check();
    env_logger::init();
    parse_args()
}

pub fn count_keystrokes(config: Config) -> std::io::Result<()> {
    let mut log_file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(config.log_file)?;
    let mut device_file = File::open(&config.device_file)?;
    let mut buf: [u8; mem::size_of::<InputEvent>() as usize] = unsafe { mem::zeroed() };
    // Reads the current count from file
    let mut current_count = {
        let mut count_str = String::new();
        
        if log_file.read_to_string(&mut count_str).is_ok() {
            count_str.parse::<u64>().unwrap_or_default()
        } else {
            0
        }
    };
    
    loop {
        // Read input event from the device
        if device_file.read(&mut buf)? != mem::size_of::<InputEvent>() {
            panic!("Error while reading from device file");
        }
        
        let event: InputEvent = unsafe { mem::transmute(buf) };
        
        if event.is_key_event() && event.is_key_press() {
            current_count += 1;
            // reset the file back to the beginning
            log_file.seek(SeekFrom::Start(0))?;
            log_file.write_all(&current_count.to_string().as_bytes())?;
        }
    }
}

#[derive(Debug)]
pub struct Config {
    device_file: PathBuf,
    log_file: PathBuf,
}

impl Config {
    fn new<T>(device_file: T, log_file: T) -> Self
    where
        T: Into<PathBuf>,
    {
        Config {
            device_file: device_file.into(),
            log_file: log_file.into(),
        }
    }
}

fn parse_args() -> Config {
    let args: Vec<_> = env::args().collect();
    let mut opts = Options::new();
    
    opts.optflag("h", "help", "prints this help message");
    opts.optflag("v", "version", "prints the version");
    opts.optopt("d", "device", "specify the device file", "DEVICE");
    opts.optopt("f", "file", "specify the file to log to", "FILE");
    
    let matches = opts.parse(&args[1..])
        .unwrap_or_else(|e| panic!("{}", e));
    
    if matches.opt_present("h") {
        // Print usage
        let program = &args[0];
        println!("{}", opts.usage(&format!("Usage: {program} [options]")));
        exit(0);
    }

    if matches.opt_present("v") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }
    
    let device_file = matches.opt_str("d")
        .unwrap_or_else(|| get_default_device());
    let log_file = matches.opt_str("f")
        .unwrap_or("keys.log".to_owned());

    Config::new(device_file, log_file)
}

fn get_default_device() -> String {
    let mut filenames = get_keyboard_device_filenames();
    debug!("Detected devices: {:?}", filenames);
    
    if filenames.len() != 1 {
        panic!("The following keyboard devices were detected: {filenames:?}. Please select one using the `-d` flag");
    }
    
    filenames.swap_remove(0)
}

// Detects and returns the name of the keyboard device file. This function uses
// the fact that all device information is shown in /proc/bus/input/devices and
// the keyboard device file should always have an EV of 120013
fn get_keyboard_device_filenames() -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("grep -E 'Handlers|EV' /proc/bus/input/devices | grep -B1 -E '120013|12001f' | grep -Eo event[0-9]+")
        .output()
        .unwrap_or_else(|e| panic!("{}", e));
    
    std::str::from_utf8(&output.stdout).unwrap()
        .trim()
        .split('\n')
        .map(|filename| format!("/dev/input/{filename}"))
        .collect()
}