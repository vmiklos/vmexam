# darcs-git

darcs-git is a [darcs](http://darcs.net/)-like porcelain on top of git plumbing:

- rec (record) is a wrapper around `git add` and `git commit`
- rev (revert) is a wrapper around `git checkout`
- what (what's new) is a wrapper around `git diff`
- push is a wrapper around `git log`, `git push` and `git pull`
- unrec (unrecord) is a wrapper around `git reset`
- unpull is also a wrapper around `git reset`
