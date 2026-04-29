# strava-mirror

Mirrors your Strava activities from <http://strava.com/> for backup purposes, incrementally.

For now, it only fetches activities: metadata and the original data.

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam strava-mirror
```

## Usage

See <https://developers.strava.com/docs/authentication/>, generate a refresh token using
<https://github.com/pR0Ps/strava-tokengen>. The original data is not available via the API, so get
your JWT value using:

- Go to strava.com in your web browser (e.g. Firefox), log in
- Open dev-tools (F12)
- Go to Storage -> Cookies -> strava.com
- Look for a key named `strava_remember_token`

The configuration file is `~/.config/strava-mirrorrc`:

```
client_id = "..."
client_secret = "..."
refresh_token = "..."
jwt = "..."
```

Once `strava-mirror` is completed, you can find your activities under
`~/.local/share/strava-mirror/activities/`.

## Querying

You can query your local activities, e.g. to see which country each activity was in:

```
strava-mirror --query countries
```

Use the `--summary` flag to only see the country counts:

```
strava-mirror --query countries --summary
```

Use the `--html` flag to get a more detailed HTML output:

```
strava-mirror --query countries --html
```

## Cron

If you want to automate downloading your activities, there is a `--quiet` option to omit the INFO
log lines, which are only interesting in the interactive case.
