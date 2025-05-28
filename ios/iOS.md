# plistServer on iOS
If you're doing plist related developement work with iOS, you may want to take a look at [this](plistServer.xcodeproj) sample Xcode project for a Rust-Swift bridge / FFI built for plistServer.

Also see a sample .ipa in [Releases](https://github.com/nekohaxx/plistserver/releases) with some simple UI that can generate plists locally.

The library is [lib.rs](lib.rs) compiled into [libplistgen.a](libplistgen.a) as compiled machine code, a C compatible header declaration at [plistgen.h](plistgen.h), and the bringing header at [BringingHeader.h](bringingheader.h).
SwiftUI simply calls it.
