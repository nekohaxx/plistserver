# plistServer on iOS
If you're doing plist related developement work with iOS, you may want to take a look at [this](plistServer.xcodeproj) sample Xcode project for a Rust-Swift bridge / FFI built for plistServer.

Also see a sample .ipa [here](Payload.ipa) that takes plistServer, bridges the Rust to Swift, takes inputs, calls plistServer through the bridge, generates and renders a preview, and allows you to export the configured .plist.

The library is [lib.rs](lib.rs) compiled into [libplistgen.a](libplistgen.a) as compiled machine code, a C compatible header declaration at [plistgen.h](plistgen.h), and the bringing header at [BringingHeader.h](bringingheader.h).
SwiftUI simply calls it.

Built by [Jacob](https://github.com/jacobprezant).


<p>
  <img src="https://github.com/user-attachments/assets/0b5f6553-99f3-42ea-bc2b-dd97e9339577" alt="IMG_4724" width="30%">
  <img src="https://github.com/user-attachments/assets/e67c0030-5916-477b-8464-f5d27ac16cc4" alt="IMG_4722" width="30%" style="margin-right: 1%">
  <img src="https://github.com/user-attachments/assets/64f0c82c-bf76-4bb8-bcb5-a5825e05a797" alt="IMG_4723" width="30%" style="margin-right: 1%">
</p>
