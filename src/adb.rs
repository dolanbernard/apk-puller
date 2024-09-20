use core::str;
use std::{fs::File, path::Path};

use adb_client::{ADBServerDevice, RustADBError};
use regex::Regex;

pub fn list_packages(device: &mut ADBServerDevice) -> Result<String, RustADBError> {
    let r = Regex::new("^package:").unwrap();
    command(device, ["pm", "list",  "packages",  "--user 0"])
        .map(|s| r.replace(&s, "").to_string())
}

pub fn get_path(device: &mut ADBServerDevice, package: impl AsRef<str>) -> Result<String, RustADBError> {
    let r = Regex::new("^package:").unwrap();
    command(device, ["pm", "path", "--user 0", package.as_ref()])
        .map(|s| r.replace(&s, "").to_string())
}

// TODO: this
/*pub fn get_app_name(device: &mut ADBServerDevice, package: impl AsRef<str>) -> Result<String, RustADBError> {
    let lines = get_path(device, package).unwrap();
    let path = lines.lines().next().unwrap();
    command(device, ["dumpsys", path.as_ref(), "dump", "badging", path.as_ref()])
}*/

pub fn pull_apk(device: &mut ADBServerDevice, path: impl AsRef<str>, output_dir: impl AsRef<str>) -> Result<(), RustADBError> {
    let mut output_file = String::from(output_dir.as_ref());
    let apk_name = Path::new(path.as_ref()).file_name().unwrap();
    output_file.push('/');
    output_file.push_str(&apk_name.to_str().unwrap());
    let mut stream = File::create(Path::new(&output_file))?;
    device.recv(path.as_ref(), &mut stream)
}

/*pub fn push_apk(device: &mut ADBServerDevice, apk_paths: &[impl AsRef<str>]) -> Result<String, RustADBError> {
    apk_paths.iter().for_each(|f| {
        let mut stream = File::create(Path::new(f));
        device.send(stream, path)
    });
}*/

fn command(device: &mut ADBServerDevice, command: impl IntoIterator<Item = impl ToString>) -> Result<String, RustADBError> {
    let mut output = vec![];
    device.shell_command(command.into_iter(), &mut output)
        .map(|_| str::from_utf8(&output).unwrap().trim().to_string())
}
