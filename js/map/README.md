# js map

This gives you a way to share unlisted (not fully public, neither fully private) GeoJSONs with your
friends, which is not something Strava provides, unless those friends register on Strava.

Example URL: <http://0.0.0.0:8000/?a=rd9mcAxgJWG3271aD0VRGjQ>

## Collections

It's also possible to group acitivities together with a JSON like this:

```
{
    "title": "mytitle",
    "activities": [
        "rd9mcAxgJWG3271aD0VRGjQ",
        "rd9mcAxgJWG3271aD0VRGjR"
    ]
}
```

Example URL: <http://0.0.0.0:8000/?c=rd9mcAxgJWG3271aD0VRGjS>
