use anyhow::Context as _;

#[derive(serde::Deserialize)]
struct Config {
    projects: Vec<String>,
}

struct Project {
    dir: std::path::PathBuf,
    config: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let home_dir = home::home_dir().context("home_dir() failed")?;
    let config_path = home_dir.join(".config").join("git-ls-projects.toml");
    let config_string = std::fs::read_to_string(&config_path)
        .context(format!("failed to read config from '{config_path:?}'"))?;
    let config: Config = toml::from_str(&config_string)?;
    let mut projects = Vec::new();
    for project in config.projects {
        let project_config = project + "/Cargo.toml";
        for result in glob::glob(&project_config)? {
            let entry = result?;
            let dir = entry.parent().context("no parent")?.to_path_buf();
            let config = entry.clone();
            let project = Project { dir, config };
            projects.push(project);
        }
    }

    for project in projects {
        let dir = project.dir.to_str().context("no str")?;
        let config = project
            .config
            .file_name()
            .context("no file name")?
            .to_str()
            .context("no str")?;
        // git log --pretty=format:%cd --date=relative -- ../darcs-git/Cargo.toml
        let args = [
            "-C",
            dir,
            "log",
            "--date=relative",
            "--pretty=format:%cd",
            "-1",
            "--",
            config,
        ];
        let output = std::process::Command::new("git").args(args).output()?;
        let date = String::from_utf8(output.stdout)?;
        println!("{}: {}", dir, date);
    }

    Ok(())
}
