import SwiftUI

struct ContentView: View {
    @State private var bundleID = ""
    @State private var name = ""
    @State private var version = ""
    @State private var fetchURL = ""
    @State private var plistResult = ""

    var body: some View {
        VStack(spacing: 0) {
            NavigationView {
                Form {
                    Section(header: Text("App Info")) {
                        TextField("Bundle ID", text: $bundleID)
                            .autocapitalization(.none)
                        TextField("App Name", text: $name)
                        TextField("Version", text: $version)
                        TextField("Fetch URL", text: $fetchURL)
                            .keyboardType(.URL)
                            .autocapitalization(.none)
                    }

                    Section {
                        Button("Generate PLIST") {
                            plistResult = generate(
                                bundleid: bundleID,
                                name: name,
                                version: version,
                                fetchurl: fetchURL
                            )
                        }
                        .disabled(bundleID.isEmpty || name.isEmpty || version.isEmpty || fetchURL.isEmpty)
                    }

                    if !plistResult.isEmpty {
                        Section(header: Text("Generated PLIST")) {
                            ScrollView {
                                Text(plistResult)
                                    .font(.system(size: 12, design: .monospaced))
                                    .padding(.vertical, 4)
                            }
                            .frame(height: 200)

                            Button("Share .plist File") {
                                sharePlist()
                            }
                        }
                    }
                }
                .navigationTitle("PLIST Generator")
            }

            Spacer()

 
            VStack(spacing: 4) {
                Text("Created by Jacob Prezant")
                    .font(.footnote)
                    .foregroundColor(.secondary)

                Link("Plistserver",
                     destination: URL(string: "https://github.com/nekohaxx/plistserver")!)
                    .font(.footnote)
            }
            .padding(.bottom, 10)
        }
    }

    func generate(bundleid: String, name: String, version: String, fetchurl: String) -> String {
        let cStr = generate_plist(bundleid, name, version, fetchurl)
        return String(cString: cStr!)
    }

    func sharePlist() {
        let filename = "output.plist"
        let url = FileManager.default.temporaryDirectory.appendingPathComponent(filename)
        try? plistResult.write(to: url, atomically: true, encoding: .utf8)

        let activityVC = UIActivityViewController(activityItems: [url], applicationActivities: nil)

        if let scene = UIApplication.shared.connectedScenes.first as? UIWindowScene,
           let window = scene.windows.first,
           let rootVC = window.rootViewController {
            rootVC.present(activityVC, animated: true)
        }
    }
}
