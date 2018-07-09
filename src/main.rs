extern crate git2;
extern crate semver;
#[macro_use]
extern crate structopt;

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
                let tag = tag.trim_left_matches('v');
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
    #[structopt(long = "patch", help = "A bugfix release (3.2.1 -> 3.2.2)")]
    fix: bool,
    #[structopt(long = "minor", help = "A normal release (3.2.1 -> 3.3.0)")]
    feature: bool,
    #[structopt(long = "major", help = "An incompatible release (3.2.1 -> 4.0.0)")]
    breaking: bool,
    #[structopt(long = "quiet", help = "Print just the version")]
    quiet: bool,
    #[structopt(long = "force", help = "Allow more than one tag for HEAD")]
    force: bool,
    #[structopt(default_value = ".", help = "Path to git repo")]
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
    if opt.fix {
        if increment {
            version.increment_patch();
        }
    } else if opt.feature {
        if increment {
            version.increment_minor();
        }
    } else if opt.breaking {
        if increment {
            version.increment_major();
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
