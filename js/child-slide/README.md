# child-slide

A simple web app that lets you select a kid from a collection, pick an age, and jump to the
corresponding video. The app reads a `?c=TOKEN` query parameter, fetches `TOKEN.json`, and presents
a UI that redirects to the video URL where the kid is closest to the selected age.

## Sample JSON

```json
{
    "https://example.com/video1.mp4": {
        "Alice": 3,
        "Bob": 1
    },
    "https://example.com/video2.mp4": {
        "Alice": 4,
        "Bob": 2,
        "Cecil": 1
    },
    "https://example.com/video3.mp4": {
        "Alice": 5,
        "Bob": 3,
        "Cecil": 2
    }
}
```
