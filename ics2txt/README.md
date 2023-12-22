# ics2txt

This is meant to provide a plain text view of an ics file, used in calendar invite email
attachments.

It's inspired by <https://github.com/terabyte/mutt-filters>, but ics2txt shows the start/end in
local time zone, preserving also the original time zone, similar to what `mutt-display-filter` does
for email.
