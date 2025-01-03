use std::{fs::OpenOptions, os::unix::fs::FileExt};
use colored::*;
use bcrl_rs::{BcrlFactory, SearchConstraints};
use byteorder::NativeEndian;
use procfs::process::all_processes;
use signature_scanner::Signature;

const VERSION: &str = "1.0.0";

fn wallhack() {
    let process = all_processes()
        .expect("Couldn't read processes")
        .filter_map(|p| p.ok())
        .find(|p| p.status().map(|s| s.name == "cs2").unwrap_or(false))
        .expect("Didn't find cs2 process. Run CS2 first!");
    let mem_file = OpenOptions::new()
        .read(true)
        .write(true) // profs doesn't support opening with write permissions
        .open("/proc/".to_owned() + &process.pid().to_string() + "/mem")
        .expect("Could't open mem file");
    let factory =
        BcrlFactory::from_files(&process.maps().expect("Couldn't open maps file"), &mem_file)
            .expect("Couldn't create bcrl factory");

    let ptr = factory
        .signature(
            Signature::string("mp_dogtag_pickup_rule", true),
            SearchConstraints::everything()
                .thats_readable()
                .with_name("libclient.so".to_owned()),
        )
        .find_all_references::<NativeEndian>(
            4,
            SearchConstraints::everything()
                .thats_executable()
                .with_name("libclient.so".to_owned()),
        )
        .next_occurrence(
            Signature::ida("4c 89 e6 4c 89 ef"),
            SearchConstraints::everything()
                .thats_readable()
                .thats_executable()
                .with_name("libclient.so".to_owned()),
        )
        .step_forwards(6)
        .step_forwards(1)
        .relative_to_absolute::<NativeEndian>()
        .step_forwards(2)
        .get_pointer()
        .expect("Couldn't find IsOtherEnemy");

    mem_file
        .write_at(&[0xc3], ptr as u64)
        .expect("Couldn't write to offset");
}

fn main() {
    println!("{}{}", r#"
▗▖ ▗▖▗▞▀▜▌ ▄▄▄  ▄▄▄ ▄ █  ▐▌▄▄▄▄  
▐▌ ▐▌▝▚▄▟▌▀▄▄  ▀▄▄  ▄ ▀▄▄▞▘█ █ █ 
▐▛▀▜▌     ▄▄▄▀ ▄▄▄▀ █      █   █ 
▐▌ ▐▌               █       "#.red(), VERSION.blue());
    println!("{} {}", "Hassium".red(),"Wallhack".green());
    println!("{}", "https://szczota.is-a.dev/".blue());
    println!("{}", "Started Hassium...".green());
    wallhack();
}