# tag-helper - A tool to increment semver-compatible git tags

[![crates.io](https://img.shields.io/crates/v/tag-helper.svg)](https://crates.io/crates/tag-helper)
[![build status](https://github.com/tshepang/tag-helper/workflows/CI/badge.svg)](https://github.com/tshepang/tag-helper/actions)

It does the following (tedious) steps in a single command:

- Listing latest tag (provided it's in semver format)
- Tagging the repo with a version that increments that tag

Following is what the `--help` option looks like:

```
Usage: tag-helper [OPTIONS] [REPO]

Arguments:
  [REPO]  Path to git repo [default: .]

Options:
      --build <BUILD>  A build-release (3.2.1 -> 3.2.1+build)
      --pre <PRE>      A pre-release (3.2.1 -> 3.2.1-beta.0)
      --patch          A bugfix release (3.2.1 -> 3.2.2)
      --minor          A normal release (3.2.1 -> 3.3.0)
      --major          An incompatible release (3.2.1 -> 4.0.0)
      --quiet          Print just the version
      --force          Allow more than one tag for HEAD
  -h, --help           Print help
  -V, --version        Print version
```

All that's left is pushing the resulting tag to remote repo (via `git push`).

NOTE: minimum required rustc is v1.64, [due to clap].

[due to clap]: https://github.com/clap-rs/clap/pull/4615

---

The code is distributed under the terms of both the
[MIT license](LICENSE-MIT) and the
[Apache License (Version 2.0)](LICENSE-APACHE)
