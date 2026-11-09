# git tzstats: DST-aware timezone stats

This tool looks at git history and looks for commits where the author has an unusual timezone,
signalling that the person traveled somewhere.

It assumes that the first commit of each calendar year is at the home location. That offset is
considered as the home timezone; later expected offsets are derived from `/etc/localtime`, so normal
DST changes are ignored. It then looks for date ranges where the timezone changes to a different
one, then back to normal, and prints the completed trips.

Days with more than one timezone are ignored, as those are likely misconfigurations. Offsets that
match the home timezone within 7 days of a DST transition are also ignored.

By default, all authors and the entire repo history is analyzed. This can be limited by the
`--author=...` and `--since=...` and `--until=...` switches, which are all passed to the underlying
`git log`. Run the tool inside the git repository you want to analyze.

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam git-tzstats
```

## Usage

```
git tzstats --author=Alice --since=2013-09-01
```

Sample output:

```
Alice traveled to +0100 on 2015-05-22 and returned to +0200 on 2015-05-26.
```
