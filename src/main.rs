use anyhow::Result;
use clap::Parser;
use git2::Repository;
use semver::Version;

#[derive(Parser)]
#[command(about, version)]
struct Opt {
    /// A build-release (3.2.1 -> 3.2.1+build)
    #[arg(long)]
    build: Option<String>,
    /// A pre-release (3.2.1 -> 3.2.1-beta.0)
    #[arg(long)]
    pre: Option<String>,
    /// A bugfix release (3.2.1 -> 3.2.2)
    #[arg(long)]
    patch: bool,
    /// A normal release (3.2.1 -> 3.3.0)
    #[arg(long)]
    minor: bool,
    /// An incompatible release (3.2.1 -> 4.0.0)
    #[arg(long)]
    major: bool,
    /// Allow more than one tag for HEAD
    #[arg(long)]
    force: bool,
    /// Path to git repo
    #[arg(default_value = ".")]
    repo: String,
}

fn latest_version(
    tags: &git2::string_array::StringArray,
    repo: &git2::Repository,
) -> (Version, bool) {
    let mut latest_version = Version::parse("0.0.0").unwrap();
    let mut increment = true;
    if let Ok(head) = repo.head() {
        let head = git2::Branch::wrap(head);
        let head_ref = head.get();
        for tag in tags.into_iter().flatten() {
            let tag_name = format!("refs/tags/{tag}");
            let tag = tag.trim_start_matches('v');
            if let Ok(version) = Version::parse(tag) {
                if let Ok(reference) = repo.find_reference(&tag_name) {
                    if &reference == head_ref && Version::parse(tag).is_ok() {
                        increment = false;
                    }
                }
                latest_version = latest_version.max(version);
            }
        }
    }
    (latest_version, increment)
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    let repo = Repository::discover(&opt.repo)?;
    let tags = repo.tag_names(None)?;
    let (mut version, mut increment) = latest_version(&tags, &repo);
    if opt.force {
        increment = true;
    }
    if let Some(build) = opt.build {
        if increment {
            version.build = semver::BuildMetadata::new(&build)?;
        }
    } else if let Some(pre) = opt.pre {
        if increment {
            version.pre = semver::Prerelease::new(&pre)?;
        }
    } else if opt.patch {
        if increment {
            version.patch += 1;
            version.pre = semver::Prerelease::EMPTY;
        }
    } else if opt.minor {
        if increment {
            version.minor += 1;
            version.patch = 0;
            version.pre = semver::Prerelease::EMPTY;
        }
    } else if opt.major {
        if increment {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
            version.pre = semver::Prerelease::EMPTY;
        }
    } else {
        if version == Version::parse("0.0.0").unwrap() {
            println!("The repository does not have a semver tag");
        } else {
            eprint!("latest version: ");
            println!("v{version}");
        }
        std::process::exit(0);
    }
    if increment {
        eprint!("new tag: ");
        println!("v{version}");
        if increment {
            let head_ref = repo.head()?;
            let head_object = head_ref.peel(git2::ObjectType::Commit)?;
            repo.tag_lightweight(&format!("v{version}"), &head_object, false)?;
        }
    } else {
        eprintln!("HEAD is already tagged: v{version}");
    }

    Ok(())
}
