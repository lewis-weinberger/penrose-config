Configuration for the [Penrose](https://github.com/sminez/penrose) window manager.

### Installation

Requires [Rust](https://www.rust-lang.org/tools/install). Install with:

```sh
cd penrose_config
cargo install --path .
```

By default this should place `penrose_config` in the `${HOME}/.cargo/bin/` directory.

If you use a display manager, you can use:
- `penrose.session` (move to `/etc/X11/Sessions/`)
- `penrose.desktop` (move to `/usr/share/xsessions/`)
- `status_bar.sh` (move to `/usr/local/bin`, might need adjusting for your specific setup)
