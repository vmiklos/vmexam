# strava-mirror

Mirrors your Strava activities from <http://strava.com/> for backup purposes, incrementally.

For now, it only fetches the metadata of an activity.

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam pushping
```

## Usage

See <https://developers.strava.com/docs/authentication/>, generate a refresh token using
<https://github.com/pR0Ps/strava-tokengen>.

The configuration file is `~/.config/strava-mirrorrc`:

```
client_id = "..."
client_secret = "..."
refresh_token = "..."
```

Once `strava-mirror` is completed, you can find your activities under
`~/.local/share/strava-mirror/activities/`.
