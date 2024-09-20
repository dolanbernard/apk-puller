use clap_derive::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ApkPullerConfig {
    #[arg()]
    pub command: String,
    #[arg(required_if_eq("command", "path"), required_if_eq("command", "name"), default_value_t = String::from(""))]
    pub apk_package: String,
    #[arg(short='p', long="apk-path", required_if_eq("command", "pull"), default_value_t = String::from(""))]
    pub apk_path: String,
    //TODO: default value
    #[arg(short='o', long="output", default_value_t = default_download_dir())]
    pub output_directory: String,
    // TODO: this
    //#[arg(required_if_eq("command", "push"), num_args = 1.., value_delimiter = ' ')]
    //pub apk_filenames: Vec<String>,
}

fn default_download_dir() -> String {
    let mut home_dir =
        home::home_dir().unwrap_or("/root".into()).to_string_lossy().into_owned();
    home_dir.push_str("/Downloads");
    return home_dir;
}
