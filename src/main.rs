use std::error::Error;
use std::process::Command;

use rmesg;
use uptime_lib;

fn halt() {
    println!("Bye...");
    Command::new("/usr/bin/sudo")
            .arg("/usr/sbin/halt")
            .status()
            .expect("failed to halt system");
}

fn main() -> Result<(), Box<dyn Error>> {
    const CLEAR_LOG: bool = false;
    const RAW_LOG: bool = false;
    let entries = rmesg::logs_iter(rmesg::Backend::DevKMsg, CLEAR_LOG, RAW_LOG)?;

    let start_time = uptime_lib::get().unwrap();

    for maybe_entry in entries {
        let entry = maybe_entry?;

        if entry.timestamp_from_system_start.unwrap_or(start_time) <= start_time {
            // Ignore events that occured before starting the daemon.
            continue; 
        }

        match entry.message.find("over-current change") {
            Some(x) => halt(),
            _ => () 
        }
    }

    Ok(())
}
