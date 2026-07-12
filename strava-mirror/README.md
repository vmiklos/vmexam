# strava-mirror

Mirrors your Strava activities from <http://strava.com/> for backup purposes, incrementally.

Compared to [strava-backup](https://github.com/pR0Ps/strava-backup), this tool uses the same REST
endpoints as the website, using just the JWT. The problem with the API is that it broke on
2026-06-30 for the free tier.

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam strava-mirror
```

## Usage

Get your JWT value using:

- Go to strava.com in your web browser (e.g. Firefox), log in
- Open dev-tools (F12)
- Go to Storage -> Cookies -> strava.com
- Look for a key named `strava_remember_token`

The configuration file is `~/.config/strava-mirrorrc`:

```
jwt = "..."
```

Once `strava-mirror` is completed, you can find your activities under
`~/.local/share/strava-mirror/activities/`.

The mirroring is incremental, only activities newer than the last local activity are fetched by
default. Use `--full-history` if you want to fetch newly added older activities or updated metadata
of existing activities.

## Querying

You can query your local activities, e.g. to see which country each activity was in:

```
strava-mirror --query countries --html
```

Several other less creative stats are available, e.g.:

```
strava-mirror --query top-walks-by-time
```

To include all stats in HTML form:

```
strava-mirror --query all
```

## Cron

If you want to automate downloading your activities, there is a `--quiet` option to omit the INFO
log lines, which are only interesting in the interactive case.
