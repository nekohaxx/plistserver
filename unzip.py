import plistlib
import zipfile
ipa_path = "signed.ipa"
with zipfile.ZipFile(ipa_path, 'r') as z:
    info_plists = [f for f in z.namelist() if f.startswith("Payload/") and f.endswith(".app/Info.plist")]
    if not info_plists:
        print("No Info.plist found inside the .ipa")
    else:
        with z.open(info_plists[0]) as f:
            plist = plistlib.load(f)
            print("Bundle ID:", plist.get("CFBundleIdentifier"))
