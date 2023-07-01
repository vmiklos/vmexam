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
