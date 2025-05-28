use std::ffi::{CStr, CString};
use libc::c_char;

static PLIST_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>items</key>
    <array>
        <dict>
            <key>assets</key>
            <array>
                <dict>
                    <key>kind</key>
                    <string>software-package</string>
                    <key>url</key>
                    <string>{fetchurl}</string>
                </dict>
            </array>
            <key>metadata</key>
            <dict>
                <key>bundle-identifier</key>
                <string>{bundleid}</string>
                <key>bundle-version</key>
                <string>{version}</string>
                <key>kind</key>
                <string>software</string>
                <key>title</key>
                <string>{name}</string>
            </dict>
        </dict>
    </array>
</dict>
</plist>"#;

#[no_mangle]
pub extern "C" fn generate_plist(
    bundleid: *const c_char,
    name: *const c_char,
    version: *const c_char,
    fetchurl: *const c_char,
) -> *mut c_char {
    let bundleid = unsafe { CStr::from_ptr(bundleid).to_string_lossy() };
    let name = unsafe { CStr::from_ptr(name).to_string_lossy() };
    let version = unsafe { CStr::from_ptr(version).to_string_lossy() };
    let fetchurl = unsafe { CStr::from_ptr(fetchurl).to_string_lossy() };

    let result = PLIST_TEMPLATE
        .replace("{bundleid}", &bundleid)
        .replace("{version}", &version)
        .replace("{name}", &name)
        .replace("{fetchurl}", &fetchurl);

    CString::new(result).unwrap().into_raw()
}