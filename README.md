A tool to increment semver-comptatible git tags

It does following (tedious) steps in a single command:

- Listing tags (which are in semver format)
- Determine which is latest
- Tagging the repo with a version that follows the latest

Following is a snippet of the `--help` option:

```
USAGE:
    tag-helper [FLAGS] [repo]

FLAGS:
        --breaking    An incompatible release (3.2.1 -> 4.0.0)
        --feature     A normal release (3.2.1 -> 3.3.0)
        --fix         A bugfix release (3.2.1 -> 3.2.2)
    -h, --help        Prints help information
    -V, --version     Prints version information

ARGS:
    <repo>    path to git repo [default: .]
```

All that's left is pushing the resulting tag to remote repo.
