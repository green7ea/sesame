# Sesame – an xdg-open alternative

*Xdg-open* decides what program to use to open a file or URL but it
can be difficult to configure, inflexible and slow. This is where
*sesame* comes in as an *xdg-open* alternative.

*Sesame* is:

- easy and intuitive to configure for a tech enthusiast,
- a bit for flexible,
- faster to execute.

By using a single JSON file as a configuration, *sesame* can be
configure using a simple text editor and leads to more predictable
results. The configuration file is read in order and the first
program that matches is used.

*Sesame*'s configuration is a bit more powerful than simply choosing
a program to use based on a type, it allows you to nest conditions
and make smarter decisions. A good example of this is:

- use *qutebrowser* to open `http` and `https` links,
- but use *mpv* to play *youtube* videos,
- or use *firefox* for certain sites.

Expressed as a valid *sesame* configuration:

```json
{
  "protocol": {
    "http,https": [
      {
        "contains": ["youtube.com/watch?", "youtu.be/watch?"],
        "use": "mpv"
      },
      {
        "contains": ["atlassian.net", "gitlab.com"],
        "use": "firefox"
      },
      "qutebrowser"
    ]
  }
}
```

*Sesame* is written in *rust* and so can be compiled to a static
executable (~684 KB) which lets it do its job by using only two files:

- the *sesame* executable,
- the configuration file.
