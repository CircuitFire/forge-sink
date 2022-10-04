use sink_server::ServerData;

use reqwest::blocking::Client;

use std::io::prelude::*;
use std::fs::{read_dir, File, remove_file};
use std::collections::HashSet;


struct Differences {
    remove: Vec<String>,
    add:    Vec<String>,
}

fn version_from_file(forge: &str) -> String {
    let segments: Vec<_> = forge.split("-").collect();

    format!("{}-forge-{}", segments[1], segments[2])
}

fn download(client: &mut Client, url: &str, path: &str) -> bool {
    let response = client.get(url).send();

    if response.is_err() || response.as_ref().unwrap().status().is_server_error() {
        println!("Failed to connect to server.");
        return false
    }

    let response = response.unwrap();

    if let Ok(mut file) = File::create(path) {
        if let Ok(_) = file.write_all(&response.bytes().unwrap()[..]) {
            return true
        }
    }

    println!("An error occurred while saving file.");
    return false
}

fn check_forge(client: &mut Client, url: &str, forge: &str) {
    let os_dependent = if cfg!(target_os = "windows") {
        (
            format!(
                "{}\\Desktop\\{}",
                std::env::var("USERPROFILE").unwrap(),
                forge
            ),
            "New forge version place on your desktop."
        )
    }
    else {
        (
            format!(
                "{}/{}",
                std::env::var("HOME").unwrap(),
                forge
            ),
            "New forge version place in your home dir."
        )
    };

    let version = version_from_file(forge);
    println!("Server's Forge version is '{}'", version);

    for dir in read_dir("./versions").unwrap() {
        if dir.unwrap().file_name().to_string_lossy() == version {
            println!("Server's Forge version already installed.");
            return
        }
    }

    println!("No installed version of forge matches server.");

    println!("Attempting to download forge form server...");

    if download(client, &format!("{}/forge", url), &os_dependent.0) {
        println!("{}", os_dependent.1);
    }
}

fn find_differences(local: &Vec<String>, server: &Vec<String>) -> Differences {
    let local_hash: HashSet<_> = local.iter().collect();
    let server_hash: HashSet<_> = server.iter().collect();

    Differences {
        add: server_hash.difference(&local_hash).map(|x| String::from(*x)).collect(),
        remove: local_hash.difference(&server_hash).map(|x| String::from(*x)).collect(),
    }
}

fn read_mod_dir() -> Vec<String> {
    if let Ok(mod_dir) = read_dir("./mods") {
        mod_dir.map(|x| x.unwrap().file_name().to_string_lossy().into_owned())
        .collect()
    }
    else {
        std::fs::create_dir("./mods").unwrap();
        Vec::new()
    }
}

fn check_mods(client: &mut Client, url: &str, server_mods: &Vec<String>) {

    let local_mods = read_mod_dir();

    let differences = find_differences(&local_mods, server_mods);

    if !differences.add.is_empty() {
        println!("Downloading missing mods");
        
        for x in &differences.add {
            println!("Downloading: {}", x);
            let _ = download(client, &format!("{}/mods/{}", url, x), &format!("./mods/{}", x));
        }
    }
    else {
        println!("No missing mods.");
    }

    if !differences.remove.is_empty() {
        println!("\nRemoving other mods if you don't want to do this close the window or press 'ctrl + c'");

        println!("\n Will remove:");
        for x in &differences.remove {
            println!("\t{}", x);
        }

        println!("\npress enter to continue.");
        let mut buffer = Vec::new();
        std::io::stdin().read(&mut buffer).unwrap();
        
        for x in &differences.remove {
            println!("removing: {}", x);
            if let Err(_) = remove_file(format!("./mods/{}", x)) {
                println!("error while removing: {}", x);
            }
        }
    }
}

fn sink(url: &str) {
    let mut client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build().unwrap();


    println!("Attempting to get Minecraft server data...");
    let response = client.get(url).send();

    if response.is_err() || response.as_ref().unwrap().status().is_server_error() {
        println!("Failed to connect to server.");
        return
    }

    println!("Data received.");

    let response: ServerData = response.unwrap().json().unwrap();

    if let Some(forge) = response.forge_version {
        check_forge(&mut client, url, &forge);
    }
    else {
        println!("Couldn't retrieve forge version.");
    }

    if let Some(mods) = response.mods {
        check_mods(&mut client, url, &mods)
    }
    else {
        println!("Couldn't retrieve mod list.");
    }
}

fn main() {
    let url = std::env::args().nth(1).expect("Requires an ip or url to run.");

    sink(&format!("https://{}", url));
    
    println!("\npress enter to exit.");
    let mut buffer = Vec::new();
    std::io::stdin().read(&mut buffer).unwrap();
}
