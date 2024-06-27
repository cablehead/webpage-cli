cli frontend for [webpage](https://crates.io/crates/webpage) crate

usage:

```nushell
~/webpage-cli: webpage-cli https://www.rust-lang.org | from json | get html.opengraph
────────────┬──────────────────────────────────────────────────────────────────────────────────────────
 og_type    │ website
            │ ─────────────┬──────────────────────────────────────────────────────────────────────────
 properties │  locale      │ en_US
            │  title       │
            │  description │ A language empowering everyone to build reliable and efficient software.
            │ ─────────────┴──────────────────────────────────────────────────────────────────────────
            │ ───┬──────────────────────────────────────────────────────────────┬───────────────────
 images     │  # │                             url                              │    properties
            │ ───┼──────────────────────────────────────────────────────────────┼───────────────────
            │  0 │ https://www.rust-lang.org/static/images/rust-social-wide.jpg │ {record 0 fields}
            │ ───┴──────────────────────────────────────────────────────────────┴───────────────────
 videos     │ [list 0 items]
 audios     │ [list 0 items]
────────────┴──────────────────────────────────────────────────────────────────────────────────────────
~/webpage-cli:
```