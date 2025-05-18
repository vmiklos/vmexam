# fedora notes

## rebuild older update from source

- e.g. problem to be solved: e.g. `matrix-synapse-1.86.0-1.fc38.x86_64` was released as an update, but
it causes <https://github.com/matrix-org/synapse/issues/15809>
- `dnf --showduplicates list matrix-synapse` says the non-updated version would be
  matrix-synapse.noarch 1.63.1-2.fc38, but the synapse DB is too new for that

One way to solve this is:

- `git clone https://src.fedoraproject.org/rpms/matrix-synapse.git`, check out the f38 branch,
  revert the last commit
- `podman run --cap-add=SYS_ADMIN` to have a container where you can build packages
- `fedpkg --release f38 mockbuild` to build the `matrix-synapse-1.85.2-1.fc38.x86_64` rpm that was
  removed by `dnf update`
- scp the rpms to the server, `dnf downgrade *.rpm` to install the previous version (that is new
  enough for the DB, old enough for the issue to be not present)

## Fedora 38 -> Fedora 39

- <https://docs.fedoraproject.org/en-US/quick-docs/upgrading-fedora-offline/> for the package upgrades

- matrix bridges:

```
virtualenv -p /usr/bin/python3 .
source ./bin/activate
pip install --upgrade mautrix-facebook
pip install setuptools
pip install aiosqlite
```

and

```
virtualenv -p /usr/bin/python3 .
source ./bin/activate
pip install --upgrade mautrix-signal
pip install aiosqlite
```

but then need to hack-around because mautrix-signal would want a too old asyncpg. Workaround is to
copy&paste its requirements.txt, drop the asyncpg line, then install mautrix-signal with --no-deps.

- package build:

```
fedpkg --release f39 mockbuild
```

## Fedora 39 -> Fedora 40

- <https://docs.fedoraproject.org/en-US/quick-docs/upgrading-fedora-offline/> for the package upgrades

## Fedora 41 -> Fedora 42

- <https://docs.fedoraproject.org/en-US/quick-docs/upgrading-fedora-offline/> for the package upgrades
- screen 5, given a [screen 4 config](https://github.com/vmiklos/dotfiles/blob/4f91eec5b6b4f2d7d7beeb17a5531888ed2b2df9/.screenrc#L3-L4), a new caption string is like this:

```
caption always
caption string "%{7;0}[%c] mymachine %-w%{0;7}%n %t%{-}%+w"
```
