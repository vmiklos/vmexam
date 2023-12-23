# weesearch

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam weesearch
```

## Usage

Simple search tool for weechat logs. Examples:

- to find all messages from a user:

```
weesearch -f Alice
```

- to find all messages from a given channel:

```
weesearch -c mychannel
```

- find messages from a given month (`all` disables the filter, defaults to the current month):

```
weesearch -d 2023-09
```

- find all imgur links in the content of messages:

```
weesearch imgur
```

And obviously these can be combined.
