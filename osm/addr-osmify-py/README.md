# addr-osmify-py

Takes an nominatim query (e.g. 'Mészáros utca 58/a, Budapest') and turns it
into a string that is readable (so that you can save it to your contacts) and
is also machine-friendly, e.g. OsmAnd can parse it as well.

This implementation is written in Python:

- [x] static typing (mypy)

- [x] consistent code formatting (flake8)

- [x] documentation (this file for users, Python doc strings for developers)

- [x] tests (100% statement coverage)

- [x] static code analysis (pylint)

## Install

```
python3.11 -m venv addr-osmify-py-env
. addr-osmify-py-env/bin/activate
pip install -r requirements.txt
```
