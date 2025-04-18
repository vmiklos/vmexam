# git-review-link

A commandline tool that takes a git commit hash and prints out the matching github pull request URL
(if there is one), similar to the PR link on github's web interface.

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam git-review-link
```

Then create a `~/.config/git-review-linkrc` with a content link this:

```
access_token='...'
```

You can create access tokens on [github's fine-grained personal access tokens
page](https://github.com/settings/personal-access-tokens).

## Usage

Clone e.g. `git@github.com:collaboraonline/online`, then:

```
$ git review-link cf83af8551e1b312103c996a38d6df04d077e33e
https://github.com/CollaboraOnline/online/pull/11683
```
