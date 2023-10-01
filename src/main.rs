mod util;

use std::fs::{File, OpenOptions};
use std::{env, io};
use reqwest::Client;
use std::fs;
use crate::util::PaperBuildVersion;
use crate::util::PaperVersion;
use std::io::{Write};

static DOWNLOAD: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // client
    let client: Client = Client::new();

    //user input
    //let mut input_result: Vec<String> = Vec::new();
    let input_result: Vec<String> = env::args().collect();
    //  loop {
    //      let mut input = String::new();
    //
    //      println!("Enter Version (Example 1.20.1) :");
    //      io::stdin().read_line(&mut input).unwrap();
    //      if input.trim().to_string() == "exit" {
    //          break;
    //      }
    //      input_result.push(input);
    //  }


    // Datei nur löschen, wenn nicht gedownloadet werden soll
    if ! DOWNLOAD {
        match fs::remove_file("hashes.yaml") {
            Err(_err) => {},
            _ => {}
        }
    }
    // alle Version durchgehen
    for input in &input_result {
        if !input.chars().next().unwrap().is_numeric() {
            continue
        }
        println!("Version: {}", input);
        handle_version(&client, input).await?;
    }

    Ok(())
}

async fn handle_version(client: &Client, version: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url_version = format!(
        "https://api.papermc.io/v2/projects/paper/versions/{}/",
        version
    );

// talk to paper -> PaperVersion
    let response_version = client.get(url_version).send().await?;
    let str = response_version.text().await?;
    let pv: PaperVersion = serde_json::from_str(&*str)?;

// get PaperBuildVersion
    let url_paperbuildversion = format!(
        "https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/",
        version,
        pv.builds.last().get_or_insert(&1)
    );

    let response_paperbuildversion = client.get(url_paperbuildversion).send().await?;
    let r2 = response_paperbuildversion.text().await?;
    let pbv: PaperBuildVersion = serde_json::from_str(&*r2)?;

// DOWNLOAD file


    if DOWNLOAD {
        download_paper(client, pbv).await.expect("Failed downloading!");
    } else {
        let url = format!(
            "https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/{}",
            pbv.version,
            pbv.build,
            pbv.downloads.application.name
        );

// Liste erstellen und Informationen hinzufügen
        let mut info_list = Vec::new();
        info_list.push(format!("{}:", version
            .replace("\n", "").replace("\r", "")));
        info_list.push(format!("  hash: {}", pbv.downloads.application.sha256));
        info_list.push(format!("  url: {}", url));

// Datei schreiben
        if let Err(err) = write_to_file("hashes.yaml", &info_list) {
            eprintln!("Fehler beim Schreiben der Datei: {}", err);
        }
    }

    Ok(())

}


fn write_to_file(filename: &str, lines: &Vec<String>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;

    for line in lines {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }

    Ok(())
}

async fn download_paper(client: &Client, pbv: PaperBuildVersion)
                        -> Result<(), Box<dyn std::error::Error>> {
    //TODO check hashsum if DOWNLOAD

    // check if file exists
    match fs::metadata(&pbv.downloads.application.name) {
        Ok(_) => println!("File already exists!"),
        Err(_) => {
            println!("Downloading {}", &pbv.downloads.application.name);
            let url = format!(
                "https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/{}",
                pbv.version,
                pbv.build,
                pbv.downloads.application.name
            );
            let resp = client.get(url).send().await?;
            let body = resp.text().await?;
            let mut out = File::create(pbv.downloads.application.name)
                .expect("failed to create file");
            io::copy(&mut body.as_bytes(), &mut out)
                .expect("TODO: panic message");
        }
    }

    Ok(())
}
