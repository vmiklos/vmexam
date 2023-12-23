# ics2txt

This is meant to provide a plain text view of an ics file, used in calendar invite email
attachments.

It's inspired by <https://github.com/terabyte/mutt-filters>, but ics2txt shows the start/end in
local time zone, preserving also the original time zone, similar to what `mutt-display-filter` does
for email.

Sample output:

```
$ ics2txt test.ics
Summary    : My summary
Description: My, description
Location   : https://www.example.com/
Organizer  : mailto:first.last@example.com
Dtstart    : Tue, 19 Dec 2023 11:00:00 +0100 (Tue, 19 Dec 2023 14:00:00 +0400)
Dtend      : Tue, 19 Dec 2023 12:00:00 +0100 (Tue, 19 Dec 2023 15:00:00 +0400)
```

Where +0100 is the local time, and +0400 is the original time.
