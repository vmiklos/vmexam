use anyhow::Context as _;

#[derive(serde::Deserialize)]
struct Config {
    destination: String,
}

fn main() -> anyhow::Result<()> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let home_dir: String = home_dir.to_str().context("to_str() failed")?.into();
    let config_path = home_dir + "/.config/ssh-proxyrc";
    let config_string = std::fs::read_to_string(&config_path)
        .context(format!("failed to read config from '{config_path}'"))?;
    let config: Config = toml::from_str(&config_string)?;
    loop {
        let args = [
            "-t",
            "-R",
            "2222:localhost:22",
            &config.destination,
            "watch date",
        ];
        std::process::Command::new("ssh").args(args).status()?;
    }
}
