use std::fs;
use std::path;
use std::process::Command;

const DOWNLOAD_DIR: &str = "webidl_src";

fn retrieve_firefox_idl() {
    // We retrieve IDL
    // Retrieve source for firefox 59.0, for exact files see
    // https://archive.mozilla.org/pub/firefox/releases/59.0/SOURCE
    let full_download_dir = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), DOWNLOAD_DIR);
    
    let release_hash = "c61f5f5ead48c78a80c80db5c489bdc7cfaf8175";
    let release_archive = format!("{}.tar.bz2", release_hash);
    let release_url = format!("https://hg.mozilla.org/releases/mozilla-release/archive/{}", release_archive);
    let release_file = format!("{}/{}", full_download_dir, release_archive);
    let unzipped_release_dir = format!("{}/mozilla-release-{}", full_download_dir, release_hash);

    // Make the download dir
    fs::create_dir_all(&full_download_dir).expect("Could not create download dir");
    

    // If we don't have a release archive, fetch it.
    if !path::Path::new(&release_file).is_file() {
        println!("Need to fetch release archive in {}...", release_file);
        let output = Command::new("curl")
            .args(&[&release_url, "-o", &release_file])
            .status()
            .expect("Could not retrieve archive");
    }

    // If we don't have an archive directory, unzip it.
    if !path::Path::new(&unzipped_release_dir).is_dir() {
        println!("We need to unzip the release file into {}...", unzipped_release_dir);
        let output = Command::new("tar")
            .args(&["-xvf", &release_file, "-C", &full_download_dir])
            .status()
            .expect("Could not extract archive");
    }

    // Ok, we have an unzipped dir, move the IDL files to somewhere useful.
    let webidl_src = format!("{}/dom/webidl", unzipped_release_dir);
    let webidl_dst = format!("{}/firefox_webidl", full_download_dir);
    fs::rename(&webidl_src, &webidl_dst).expect("Could not copy files");
}

fn main() {
    retrieve_firefox_idl();
}
