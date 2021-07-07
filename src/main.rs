use git2::Repository;
use semver::Version;
use structopt::StructOpt;

fn latest_version(
    tags: &git2::string_array::StringArray,
    repo: &git2::Repository,
) -> (Version, bool) {
    let mut latest_version = Version::parse("0.0.0").unwrap();
    let mut increment = true;
    if let Ok(head) = repo.head() {
        let head = git2::Branch::wrap(head);
        let head_ref = head.get();
        for tag in tags {
            if let Some(tag) = tag {
                let tag_name = format!("refs/tags/{}", tag);
                let tag = tag.trim_start_matches('v');
                if let Ok(version) = Version::parse(tag) {
                    if let Ok(reference) = repo.find_reference(&tag_name) {
                        if &reference == head_ref && Version::parse(&tag).is_ok() {
                            increment = false;
                        }
                    }
                    latest_version = latest_version.max(version);
                }
            }
        }
    }
    (latest_version, increment)
}

#[derive(StructOpt)]
#[structopt(about = "A tool to increment semver-comptatible git tags")]
struct Opt {
    /// A bugfix release (3.2.1 -> 3.2.2)
    #[structopt(long)]
    patch: bool,
    /// A normal release (3.2.1 -> 3.3.0)
    #[structopt(long)]
    minor: bool,
    /// An incompatible release (3.2.1 -> 4.0.0)
    #[structopt(long)]
    major: bool,
    /// Print just the version
    #[structopt(long)]
    quiet: bool,
    /// Allow more than one tag for HEAD
    #[structopt(long)]
    force: bool,
    /// Path to git repo
    #[structopt(default_value = ".")]
    repo: String,
}

fn tagger() -> Result<(), git2::Error> {
    let opt = Opt::from_args();
    let repo = Repository::discover(&opt.repo)?;
    let tags = repo.tag_names(None)?;
    let (mut version, mut increment) = latest_version(&tags, &repo);
    if opt.force {
        increment = true;
    }
    if opt.patch {
        if increment {
            version.patch += 1;
        }
    } else if opt.minor {
        if increment {
            version.minor += 1;
            version.patch = 0;
        }
    } else if opt.major {
        if increment {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
        }
    } else {
        if version == Version::parse("0.0.0").unwrap() {
            println!("The repository does not have a semver tag");
        } else if opt.quiet {
            println!("v{}", version);
        } else {
            println!("latest version: v{}", version);
        }
        std::process::exit(0);
    }
    if increment {
        if opt.quiet {
            println!("v{}", version);
        } else {
            println!("new tag: v{}", version);
        }
        if increment {
            let head_ref = repo.head()?;
            let head_object = head_ref.peel(git2::ObjectType::Commit)?;
            repo.tag_lightweight(&format!("v{}", version), &head_object, false)?;
        }
    } else {
        eprintln!("HEAD is already tagged: v{}", version);
    }

    Ok(())
}

fn main() {
    if let Err(why) = tagger() {
        eprintln!("{}", why);
        std::process::exit(1);
    }
}
