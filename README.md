# Slider

Yew app for slideshows built with [Trunk].

### TODOS
1. ~~Props in slideshow constructor, move data to main~~
2. ~~Keyboard events for navigation (Looks pretty traightforward actually: https://github.com/yewstack/yew/issues/372#issuecomment-1010349322)~~
3. Serde for slideshows
    (Perhaps: https://github.com/serde-rs/json)
4. UI for upload of slideshow files

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.


### License

The template ships with both the Apache and MIT license.
If you don't want to have your app dual licensed, just remove one (or both) of the files and update the `license` field in `Cargo.toml`.

There are two empty spaces in the MIT license you need to fill out: `` and `Kaden Taylor <kadenjtaylor@gmail.com>`.

[trunk]: https://github.com/thedodd/trunk
