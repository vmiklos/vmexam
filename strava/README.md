# Strava notes

## backup

`strava-mirror` can now do this. <https://github.com/pR0Ps/strava-backup> worked for a while, but now it fails with:

```
2026-03-26 20:02:06,190 [   ERROR] stravabackup: Failed to parse provided JWT '...' - ignoring it
Traceback (most recent call last):
  File "/usr/local/lib/python3.13/site-packages/stravabackup/__init__.py", line 146, in _validate_jwt
    data = json.loads(base64.b64decode(payload, validate=True))
                      ~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^
  File "/usr/lib64/python3.13/base64.py", line 88, in b64decode
    return binascii.a2b_base64(s, strict_mode=validate)
           ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^
binascii.Error: Excess padding not allowed
```

on openSUSE Leap 16.0.

## strava-publish

Once your Strava data is backed up locally, you can publish the last backed up activity:

```
cd fit2json/tools
./publish.sh
```
