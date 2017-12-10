# manpages

A library to build MAN pages from Markdown or DocBook documents. The conversion
is performed using [Pandoc](http://pandoc.org) and
[xlstproc](http://xmlsoft.org/XSLT/xsltproc2.html).

## Using manpages

First, you'll want to both add a build script for your crate (`build.rs`) and
also add this crate to your `Cargo.toml` via:

```toml
[build-dependencies]
manpages = "0.2"
```

Next up, assuming the MAN pages input files are in the ``man`` directory, you'll
want to write a build script like so:

```rust,no_run
// build.rs

extern crate manpages;

use std::env;
use std::path::PathBuf;
use manpages::build;

fn main() {
    let mut dst_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    dst_dir.push("man");
    build("man", &dst_dir).ok().expect("Failed to build MAN pages");
}
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
