### A full guide to installing .ipa files on iOS with an Apple Developer Program Membership, with any operating system, utilizing solely Apple and OSS tools; with plistServer.

Make an Apple ID and enroll into the Apple Developer program. Get started at [developer.apple.com](https://developer.apple.com).

**The rest of this guide assumes you have a valid Apple ID that is in the paid Apple Developer Program. Setting up such an account is not covered here in depth.**

1. Install openssl. This is likely preinstalled on your computer. 
2. Run `openssl genrsa -out csr.key 2048` in your terminal of choice to generate the first file you need (csr.key). 
3. Run `openssl req -new -key csr.key -out csr.csr`. You will be prompted to enter your 2 letter country code, your state/province name, and the name of your locality. Leave the organization name, the organizational unit name, and the common name blank. Enter an email address when prompted. **Enter a password when prompted; this can be anything, remember it.** Now you have your csr.csr file. 
4. Go to the [Apple Developer Portal](https://developer.apple.com), and log in with your developer program Apple ID. Select Certificates from “Certificates, IDs & Profiles”, click the blue plus icon, and select Apple Distribution. Upload your Certificate Signing Request (csr.csr), and download your certificate (distribution.cer).
5. Run the following command but change the information within brackets to your own paths, and get rid of the brackets. `openssl x509 -in <path/to/cer.cer> -inform DER -out distribution.pem -outform PEM` This will generate your .pem file (distribution.pem).
6. Run this code replacing the paths (for the .key we generated at the beginning and the .pem we just made) for your own and removing the brackets. This will require the password you set for your .key file, you will be prompted to enter it. `openssl pkcs12 -export -inkey </path/to/key.key> -in <path/to/pem.pem> -out distribution.p12` This will generate your .p12 file (distribution.p12).
7. Go back to the [Apple Developer Portal](https://developer.apple.com), under “Certificates IDs & Profiles” select Identifiers, click the blue + icon, and select App IDs. Select App, and ensure that the Bundle ID is Explicit rather than Wildcard. Make up a reverse-domain string and enter it as the Bundle ID; e.g. com.something.somethingelse (change "something" and "somethingelse" being two distinct strings). Enable any Capabilities you want (such as Push Notifications), and put something in as a description (e.g. My App ID). Click Continue, and then Register.
8. Get your device's UDID. There are several ways to do this, the specifics of which are not covered in depth within this guide. On MacOS we can use Finder, and a cross-platform solution is using the open source libimobiledevice.<br><br>
If using libimobiledevice, after connecting your iOS device to your computer and installing the library:
`idevice_id -1`
`ideviceinfo -u (device ID obtained previously)`
Look for the "UniqueDeviceID:" field and find your UDID there.
9. Go back to the [Apple Developer Portal](https://developer.apple.com), under “Certificates IDs & Profiles” select Devices, click the blue + icon, name your device, and enter in its UDID.
10. Go to the [Apple Developer Portal](https://developer.apple.com), under “Certificates IDs & Profiles” select Profiles, click the blue + icon, and “Register a New Provisioning Profile” under Ad Hoc distribution. Select the App ID we just created, and click Continue. Select the Certificate we created earlier, name your Provisioning Profile, click Generate, and download your .mobileprovision file (My_Provisioning_Profile.mobileprovision). You may be prompted so select your Device during this process.
11. Now, we need to sign our .ipa. All of the above is needed to  get the files that will let us do this, making an .ipa the phone will let you download, that is unique to that specific  device. The best way to do that is with the open source [zsign](https://github.com/zhlynn/zsign). Installation varies depending on the operating system, so check their README for more info and a guide. Once installed, we need to sign our .ipa:
```
zsign \
  -k <your_cert.p12path> \
  -p 'your_p12_password' \
  -m <your_profile.mobileprovisionpath> \
  -o signed.ipa \
  <unsigned.ipapath>
```
Remember to replace the paths for your own. All paths are within angle brackets. This will generate our signed.ipa.

12. Chores; we need to get a couple things set up on the iOS device before we continue. First, get your computer’s ip. E.g. `ipconfig getifaddr en0`. Remember this for later. It will be used in many of the following steps. Before we continue, make sure your iPhone is on Developer Mode. Skip this if not on iOS 16+ or if you’ve already done so.<br>

Open Settings
<br>
Go to Privacy & Security and find Developer Mode
<br>
Toggle Developer Mode on, then press Reboot
<br>
After the device reboots, unlock it, and confirm that you want to enable Developer Mode
<br>
If you have a passcode enabled, enter it.<br><br>
Now, we need to install mkcert, an open source tool for making locally-trusted development certificates. Check [their readme](https://github.com/FiloSottile/mkcert#) for install instructions. You may need to install a package manager first, depending on your computer’s OS. Assuming that mkcert is now installed, run `mkcert -install`; this will generate you a “rootCA.pem” file. We need this as for the rest of this guide we will be setting up some local servers (on our wifi network; make sure the iOS device and computer are on the same WiFi) that Apple requires for the install. The location of this file will vary depending on the OS; on Mac, its `/Users/NAME/Library/Application Support/mkcert`. Grab this file, along with your distribution.p12 file, and send them to your iOS device individually. Airdrop is the easiest, but email also works.

When you open them on iOS, you’ll get a prompt saying “Profile Downloaded”. Open Settings, navigate to General, scroll down to VPN & Device Management. Select the profile under “DOWNLOADED PROFILE”, click install in the top right, enter your passcode, click install again, and click install one more time (in red, at the bottom of your screen). Now click Done, and do the same with the other file (your rootCA.pem, and your Distribution.p12). You will be prompted to enter the password we set in the very beginning when installing your .p12. Now, navigate to General, About, and scroll to the bottom where it says “Certificate Trust Settings”. Toggle Full Trust on for your mkcert. You’ll get a popup verifying you want to continue. Once all that’s done, you’re good to continue. **If something goes wrong in this step, you will not be able to successfully install the .ipa. I recommend double checking all of the above is configured correctly.**

13. For the rest of this guide, we will be setting up the actual download and installation. This is done by a special url (itms-services) entered into Safari on iOS, which tells the device where to get a special file called a plist. This file contains information about the app (the name, version, etc.) along with the download location of the app itself. We need to do two things- generate the plist, and query the download with the plist. For this to work, the download link to the plist has to be accessed via an encryption standard called HTTPS, and the plist has to be formatted a very specific way. First thing we need to do is set up an HTTPS server on our computer to host the .plist and the .ipa download link the .plist directs iOS to. First, we need to generate some files necessary to set up the HTTPS server. Grab your ip from earlier and run `mkcert youriphere`, substituting your ip. This will generate you yourip.pem & yourip-key.pem. We’ll need these for the next step.

14. The following assumes you have python installed on your system. It is likely stock. We now want to run a python script for the HTTPS server. You can find a sample script attached to this repository [(https.py)](https.py); download it and replace YOURIP with your own computer’s ip address. Make sure all these files (yourip.pem, yourip-key.pem, signed.ipa, plist.plist once we generate it) are in the same directory as the one from which your are running your HTTPS python script. Run the script.

15. We also need to generate our plist, an easy way to do this is with the Open Source [plistServer](https://github.com/nekohaxx/plistserver), and referencing their documentation having also installed Rust, run `cargo run`. This will open a web server running locally at port 3788. We need to run a curl to our plistServer to generate our plist. But first, we need to do a few things. <br><br>
`curl "http://127.0.0.1:3788/genPlist?bundleid=YOURBUNDLEID&name=YourApp&version=1.0&fetchurl=https://YOURIP:4443/signed.ipa”`

Taking the above code, replace YOURIP for your own IP, YourApp for the name of the app you’re installing, and we need to replace YOURBUNDLEID with the bundle id of the ipa. Don’t replace 127.00.0.1 with your ip. **This must be exact, or else you will run into issues. It is NOT the bundle id we set earlier in the Apple Developer portal.** We’ll run a python script in this repository [(unzip.py)](unzip.py) to get the bundle id of our signed ipa. If your signed.ipa is in the same directory from which you run this script, you shouldn’t need to change anything. After running, your output will be something like `Bundle ID: thewonderofyou.Feather`. 
<br>
It doesn’t matter if this isn’t a proper reverse domain string. Note this for the next step. 
<br>
Keep in mind that this assumes your signed.ipa is in the same directory as your plistServer, and that its name is exactly “signed.ipa”. Now, we change YOURBUNDLEID to the bundle ID we just got with our python script. A sample curl is shown `curl "http://192.0.0.1:3788/genPlist?bundleid=thewonderofyou.Feather&name=Feather&version=1.0&fetchurl=https://YOURIP:4443/signed.ipa"`. Run this and expect an output like the sample in this repository [(sampleoutput.txt)](sampleoutput.txt). Copy and paste the output beginning with “<“ and ending with “>” into a text editor of your choice, and save the file as plist.plist into the same directory as everything else.
<br><br>
**This is essentially the end. Next and last step is installation. Best practice is to restart your iPhone now before you continue. Highly recommended.**<br><br>

16. Run this url in safari on your iOS device. Change YOURIP to your actual IP address.
```
itms-services://?action=download-manifest&url=https://YOURIP:4443/plist.plist
```
**This should go without saying, but make sure your iPhone and computer are on the same network.**
After entering that URL into your browser, you’ll receive a request to open this page in iTunes, and once accepted, a request to install the app from your computer’s IP. This may take a minute depending on your network speed, but if you go to your Home Screen, you should now see your app installed!
