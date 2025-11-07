# Strava backup notes

## strava-backup

<https://github.com/pR0Ps/strava-backup> works fine, but it needs access to your JWT token that
expires every 30 days. How to update it:

- Go to Chrome
- Private mode
- Open dev-tools
- Go to Application -> Cookies -> strava.com
- Look for a key named `strava_remember_token`
- Copy the value to `~/.config/strava-backup.conf`, so its `user` section will look like this:

```
[user]
email=...
# no password= row
jwt=...
```

## strava-publish

Once your Strava data is backed up locally, you can publish the last backed up activity:

```
cd fit2json/tools
./publish.sh
```
