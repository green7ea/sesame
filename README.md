# Sesame – an xdg-open alternative

When you double click on a file, *xdg-open* decides what program to
use to open it. __*xdg-open* is difficult to configure, inflexible and
slow__. This is where __*sesame*__ comes in as an __*xdg-open*
alternative__.

*Sesame* is:

- configured with a __single JSON file__,
- able to have __complex rules including regex__,
- __faster__.

By using a single JSON file as a configuration, it's __easy to
predict__ which program will be used and __easy to change__. Not only
that but *sesame* is a __single executable__ that open a __single
configuration__ file so it's pretty __lightweight__.

*Sesame*'s __configuration__ is a bit more __powerful__ than simply
choosing a program to use based on a type, it allows you to __nest
conditions__ and make smarter decisions. The __first condition that
passes__ determines which program is used to open the file. A good
example of this is:

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

This is less flexible than the shell script that is often used but
makes it faster and easier to understand.
