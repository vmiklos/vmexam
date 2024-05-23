# git-ls-projects

Tool to list projects (Rust packages as a start) in a Git repo or in other repos next to the current
one.

Sample `~/.config/git-ls-projects.toml`:

```
projects = ["**", "../extern1"]
```

`git-ls-projects` will then ask for project files (`Cargo.toml`) in those directories and will list
when it was the last time they were touched. This helps finding e.g. projects not touched for more
than a year, where a general update of outdated dependencies would be useful.
