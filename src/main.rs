use adb_client::ADBServer;
use clap::Parser;

mod adb;
mod config;

fn main() {
    let config = config::ApkPullerConfig::parse();

    let mut server = ADBServer::default();
    let mut device = server.get_device().expect("Could not connect to a device");
    match config.command.to_lowercase().trim() {
        "list" => println!("{}", adb::list_packages(&mut device).unwrap()),
        //"name" => println!("{}", adb::get_app_name(&mut device, &config.apk_package).unwrap()),
        "path" => println!("{}", adb::get_path(&mut device, &config.apk_package).unwrap()),
        "pull" => adb::pull_apk(&mut device, &config.apk_path, &config.output_directory).unwrap(),
        //"push" => adb::push_apk(&mut device, &["", ""]),
        other => println!("Invalid command: {other}"),
    }
}
