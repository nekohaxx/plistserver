# plistServer-free
A Rust server used for QuickSign to serve install manifests under HTTPS (as required by iOS)

This is licensed under the GPLv3 so you are free to use this anywhere (as long as it's under the GPLv3) under the terms of the GPLv3

## How to run
1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

2. Go into the server's cloned directory, and simply run `cargo run`

## Guide
Take a look at the [guide](Guide.md) for local .ipa signing & installation using plistServer and other open source tools.

## iOS
If you're doing plist related dev work on iOS, see [here](ios.md) for more info; including a Swift-Rust bridge and POC .xcodeproj, along with an .ipa in Releases for a local on-device app.

## Credits
plistServer was originally written by the QuickSign team.
[Source](https://github.com/QuickSign-Team/plistserver)
[c22dev](https://github.com/c22dev)
[boredcoder411](https://github.com/boredcoder411)
