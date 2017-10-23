# tag-helper - A tool to increment semver-compatible git tags

[![Linux build status](https://travis-ci.org/panoptix-za/tag-helper.svg?branch=master)](https://travis-ci.org/panoptix-za/tag-helper)

It does the following (tedious) steps in a single command:

- Listing latest tag (provided it's in semver format)
- Tagging the repo with a version that increments that tag

Following is what the `--help` option looks like:

```
USAGE:
    tag-helper [FLAGS] [repo]

FLAGS:
        --major      An incompatible release (3.2.1 -> 4.0.0)
        --minor      A normal release (3.2.1 -> 3.3.0)
        --patch      A bugfix release (3.2.1 -> 3.2.2)
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <repo>    Path to git repo [default: .]
```

All that's left is pushing the resulting tag to remote repo (via `git push`).

---

The code is distributed under the terms of both the
[MIT license](LICENSE-MIT) and the
[Apache License (Version 2.0)](LICENSE-APACHE)
