# kaps

Minimal and performant screenshot application for Xorg desktops.

### Features

- Save screenshots as png.
- Select screen region to be captured on-the-fly.

### Planned features

- Feature-flagged on-the-fly region selection.
- Pass a predetermined region to capture.
- Freeze screen while taking screenshot.

### Dependencies

kaps requires the [`slop`](https://github.com/naelstrof/slop) executable to be available on your system.

### Installation

```
$ cargo install --git https://github.com/archerfur/kaps.git
```

### Hacking

It is recommended you use an external program, such as your window manager, to extend kaps' functionality, such as:

- Defining a keybind to take a screenshot
- Including the date and time to the screenshot file name
- Copying the output file after taking the screenshot
- Automatically upload the image to a platform like imgur.

### License

This Software is dual-licensed under the MIT License ([LICENSE-MIT](./LICENSE-MIT) or [http://opensource.org/licenses/MIT]) or the Apache-2.0 license ([LICENSE-APACHE](./LICENSE-APACHE) or [https://www.apache.org/licenses/LICENSE-2.0]).  


