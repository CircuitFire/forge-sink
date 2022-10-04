use sink_server::ServerData;

use std::fs;

use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::dev::ConnectionInfo;
use actix_files::NamedFile;


use rcgen::generate_simple_self_signed;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};



#[get("/forge")]
async fn get_forge(conn: ConnectionInfo) -> actix_web::Result<NamedFile> {
    println!("forge download request from {}.", conn.peer_addr().unwrap_or("no address"));

    if let Some(forge) = get_forge_version() {
        let dir = format!("./forge/{}", forge);
        if let Ok(forge) = NamedFile::open(dir) {
            return Ok(forge)
        }
    }

    Err(actix_web::error::ErrorNotFound("forge file not found"))
}

#[get("/mods/{mod}.jar")]
async fn get_mod(conn: ConnectionInfo, req: web::Path<String>) -> actix_web::Result<NamedFile> {
    println!(
        "mod download request of {} from {}.", 
        req,
        conn.peer_addr().unwrap_or("no address")
    );

    let req_file = format!("{}.jar", req);
    
    if let Some(mods) = get_mod_list() {

        for file in mods {
            if file == req_file {
                let dir = format!("./mods/{}", file);
                if let Ok(file) = NamedFile::open(dir) {
                    return Ok(file)
                }
            }
        }
    }

    Err(actix_web::error::ErrorNotFound("forge file not found"))
}

fn get_forge_version() -> Option<String> {
    if let Ok(mut dirs) = fs::read_dir("./forge") {
        if let Some(Ok(forge)) = dirs.next() {
            return Some(forge.file_name().to_string_lossy().into_owned())
        }
    }

    None
}

fn get_mod_list() -> Option<Vec<String>> {
    if let Ok(dirs) = fs::read_dir("./mods") {
        let mods: Vec<_> = dirs.map(|x| x.unwrap().file_name().to_string_lossy().into_owned()).collect();
        return Some(mods)
    }

    None
}

#[get("/")]
async fn server_config(conn: ConnectionInfo) -> impl Responder {
    println!("server mod list request from {}.", conn.peer_addr().unwrap_or("no address"));

    let data = ServerData {
        forge_version: get_forge_version(),
        mods: get_mod_list(),
    };
    //HttpResponse::Ok().body("hello https!")

    web::Json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_rustls_config();

    println!("running.");
    HttpServer::new(|| {
        App::new()
            .service(server_config)
            .service(get_forge)
            .service(get_mod)
    })
    .bind_rustls(("0.0.0.0", 25566), config)?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    //let cert = &mut BufReader::new(File::open("./private/cert.pem").unwrap());
    //let key = &mut BufReader::new(File::open("./private/key.pem").unwrap());

    let subject_alt_names = vec!["hello.world.example".to_string(), "localhost".to_string()];
    let new_cert = generate_simple_self_signed(subject_alt_names).unwrap();

    let cert_temp = new_cert.serialize_pem().unwrap().into_bytes();
    let mut cert = &cert_temp[..];
    let key_temp = new_cert.serialize_private_key_pem().into_bytes();
    let mut key = &key_temp[..];

    // convert files to key/cert objects
    let cert_chain = certs(&mut cert)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

        
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(&mut key)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}