use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct PlistQuery {
    bundleid: String,
    name: String,
    version: String,
    fetchurl: String,
}

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
                    <string>{fetchurl}/tempsigned.ipa</string>
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

async fn generate_plist(query: web::Query<PlistQuery>) -> impl Responder {
    let plist_xml = PLIST_TEMPLATE
        .replace("{bundleid}", &query.bundleid)
        .replace("{version}", &query.version)
        .replace("{name}", &query.name)
        .replace("{fetchurl}", &query.fetchurl);

    HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(plist_xml)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/genPlist", web::get().to(generate_plist))
    })
    .bind("127.0.0.1:3788")?
    .run()
    .await
}
