# Markdown notes

I used [asciidoc](https://asciidoc.org/) for a long time for lightweight markup needs. Some reasons
to prefer [markdown](https://spec.commonmark.org/0.30/) over asciidoc:

- phabricator only accepts markdown
- same for mattermost
- same for github
- same for element

For the markup itself, markdown has one single benefit for me: if you have a list like this:

- outer list item
  - inner list item

This does what I mean for markdown, but not for asciidoc.

There are also a few drawbacks:

- The default output is much more simple, asciidoctor output can go to a static webpage as-is, the
  markdown one is more like a library output, you still need to style it manually
- Even non-numbered image captions need the `markdown_captions` extension
- The `toc` extension is needed to be able to refer to headings

For me, the list syntax was the most convincing one, given that I type such lists every day, and a
syntax like:

```
* outer
** inner
```

is not something I want to start typing on a daily basis.

Pelican uses python3-Markdown, which supports extensions: syntax highlight is part of the default
config, and the top 2 pain points are fixed with `markdown_captions` plus `toc`.
