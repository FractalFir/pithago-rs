use clap::*;
const ERR_PROJECT_ALREADY_EXISTS: i32 = 1;
#[derive(Clone, Args, Debug)]
struct NewArgs {
    name: String,
    #[arg(short, long, default_value_t = false)]
    // Will try to create a new project anyway, even if one exists already.
    force: bool,
}
#[derive(Clone, ValueEnum, Debug)]
enum BuildMode {
    Debug,
    Release,
}
#[derive(Clone, Args, Debug)]
struct BuildArgs {
    mode: BuildMode,
}
#[derive(Clone, Parser, Debug)]
enum Command {
    New(NewArgs),
    Build(BuildArgs),
    Run(BuildArgs),
}
#[derive(Debug)]
enum CommandExitStatus {
    IoError(std::io::Error),
}
impl From<std::io::Error> for CommandExitStatus {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
impl Command {
    fn execute(&self) -> Result<(), CommandExitStatus> {
        match self {
            Command::New(args) => args.execute(),
            _ => todo!("Can't handle {self:?} yet."),
        }
    }
}
impl NewArgs {
    fn execute(&self) -> Result<(), CommandExitStatus> {
        let target_path = {
            let mut tmp = std::env::current_dir()?;
            tmp.push(&self.name);
            tmp
        };
        let lockfile_path = {
            let mut tmp = target_path.clone();
            tmp.push("PITHAGO_LOCK");
            tmp.set_extension("LOCKFILE");
            tmp
        };
        if lockfile_path.exists() && !self.force {
            eprintln!("There is already a project at \"{target_path}\". Trying to create a project where there is one already is a bad idea wihich may lead to all sots of issues. You should propably eiter delete the previous project, or pick a diffrent name. If you are sure you know what you are doing, use --force to proceed anyway.",
                target_path = target_path.display());
            std::process::exit(ERR_PROJECT_ALREADY_EXISTS);
        } else if lockfile_path.exists() && self.force {
            println!("WARNING: There is already a project at \"{target_path}\". Since --force was passed, proceding anyway.",
                target_path = target_path.display());
        }
        std::fs::create_dir_all(target_path)?;
        let lockfile = std::fs::File::create(lockfile_path)?;
        todo!();
    }
}
fn main() {
    let ck = Command::parse();
    ck.execute().unwrap();
}
