#![feature(ord_max_min)]

extern crate semver;
extern crate git2;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use semver::Version;
use git2::Repository;
use structopt::StructOpt;

fn latest_version(tags: &git2::string_array::StringArray) -> Version {
    let mut latest_version = Version::parse("0.0.0").unwrap();
    for tag in tags.iter() {
        if let Some(tag) = tag {
            if let Ok(version) = Version::parse(tag.trim_left_matches('v')) {
                latest_version = version.max(latest_version);
            }
        }
    }
    latest_version
}

#[derive(StructOpt)]
#[structopt(about = "A tool to increment semver-comptatible git tags")]
struct Opt {
    #[structopt(long = "fix", help = "A bugfix release (3.2.1 -> 3.2.2)")]
    fix: bool,
    #[structopt(long = "feature", help = "A normal release (3.2.1 -> 3.3.0)")]
    feature: bool,
    #[structopt(long = "breaking", help = "An incompatible release (3.2.1 -> 4.0.0)")]
    breaking: bool,
    #[structopt(default_value = ".", help = "Path to git repo")]
    repo: String,
}

fn tagger() -> Result<(), git2::Error> {
    let opt = Opt::from_args();
    let repo = Repository::open(&opt.repo)?;
    let tags = repo.tag_names(None)?;
    let mut version = latest_version(&tags);
    if opt.fix {
        version.increment_patch();
        println!("new tag: {}", version);
    } else if opt.feature {
        version.increment_minor();
        println!("new tag: {}", version);
    } else if opt.breaking {
        version.increment_major();
        println!("new tag: {}", version);
    } else {
        if version == Version::parse("0.0.0").unwrap() {
            println!("The repository does not have a semver tag");
        } else {
            println!("latest version: {}", version);
        }
        std::process::exit(0);
    }
    let head_ref = repo.head()?;
    let head_object = head_ref.peel(git2::ObjectType::Commit)?;
    repo.tag_lightweight(&format!("v{}", version), &head_object, false)?;
    Ok(())
}

fn main() {
    if let Err(why) = tagger() {
        eprintln!("{}", why);
        std::process::exit(1);
    }
}
