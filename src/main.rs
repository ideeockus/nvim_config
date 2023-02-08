#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};
use actix_multipart::Multipart;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
use uuid::Uuid;
use futures_util::StreamExt as _;
use serde::Deserialize;

mod certificate_authority;
mod endpoints;

#[derive(Debug)]
struct KeyPair {
    uuid: Uuid,
    keypair_name: String,
}

lazy_static! {
    static ref INMEMORY_STORAGE: Mutex<HashMap<Uuid, KeyPair>> = {
        Mutex::new(HashMap::new())
    };
}

#[get("/create_keypair")]
async fn create_keypair() -> impl Responder {
    println!("Create keypair request!");
    let new_uuid = Uuid::new_v4();

    let key_pair = KeyPair {
        uuid: new_uuid,
        keypair_name: format!("{new_uuid}.keypair"),
    };

    certificate_authority::gen_keypair(&key_pair.keypair_name);

    // let mut storage: MutexGuard<HashMap<Uuid, KeyPair>> = INMEMORY_STORAGE.lock().unwrap();
    let mut storage_guard = match INMEMORY_STORAGE.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    storage_guard.insert(new_uuid, key_pair);

    println!("Debug keypairs:\n{:?}", storage_guard);

    HttpResponse::Ok().body(format!("created! {new_uuid}"))
}


// #[post("/sign_file")]
// async fn sign_file(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

#[derive(Deserialize)]
struct UploadAndSignRequest {
    uuid_str: String,
}

// async fn upload(uuid: Uuid, mut payload: Multipart) -> impl Responder {
#[post("/upload_file_and_sign")]
async fn upload_file_and_sign(req: web::Query<UploadAndSignRequest>, mut payload: Multipart) -> Result<HttpResponse, Error> {
    println!("Upload and sign request: {:?}", req.uuid_str);
    // iterate over multipart stream
    // fs::create_dir_all(UPLOAD_PATH)?;
    // let mut filename = "".to_string();
    let mut data_to_sign: Vec<u8> = Vec::new();
    let uuid = Uuid::from_str(&req.uuid_str).expect("Unable parse UUID");

    while let Some(item) = payload.next().await {
        let mut field = item?;
        // let content_type = field.content_disposition().unwrap();
        // filename = format!("{} - {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(), content_type.get_filename().unwrap(), );
        // let filepath = format!("{}/{}", UPLOAD_PATH, sanitize_filename::sanitize(&filename));
        // File::create is blocking operation, use thread pool
        // let mut f = web::block(|| std::fs::File::create(filepath))
        //     .await
        //     .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            for b in data.bytes() {
                data_to_sign.push(b.unwrap());
            }
            // data_to_sign.push(data.bytes());
            // filesystem operations are blocking, we have to use thread pool
            // f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    // TODO sign
    println!("Got data:\n{:?}", data_to_sign.len());

    // let mutex_guard = INMEMORY_STORAGE.lock().unwrap();
    let storage_guard = match INMEMORY_STORAGE.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let keypair = storage_guard.get(&uuid).expect("Unknown UUID");
    // certificate_authority::sign_data(data_to_sign, keypair);
    let signature = certificate_authority::sign_data_with_key(
        data_to_sign, &keypair.keypair_name).expect("Signing error");

    println!("Signature: {:?}", signature);

    Ok(HttpResponse::Ok().body(hex::encode(signature)).into())
}


#[derive(Deserialize)]
struct UploadAndVerifyRequest {
    uuid_str: String,
    signature: String,
}

#[post("/upload_file_and_verify")]
async fn upload_file_and_verify(req: web::Query<UploadAndVerifyRequest>, mut payload: Multipart) -> Result<HttpResponse, Error> {
    println!("Upload and verify request: {:?}", req.uuid_str);
    let mut data_to_verify: Vec<u8> = Vec::new();
    let uuid = Uuid::from_str(&req.uuid_str).expect("Unable parse UUID");

    while let Some(item) = payload.next().await {
        let mut field = item?;
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            for b in data.bytes() {
                data_to_verify.push(b.unwrap());
            }
        }
    }

    println!("Got data:\n{:?}", data_to_verify.len());

    // let mutex_guard = INMEMORY_STORAGE.lock().unwrap();
    let storage_guard = match INMEMORY_STORAGE.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let keypair = storage_guard.get(&uuid).expect("Unknown UUID");
    // certificate_authority::sign_data(data_to_sign, keypair);

    let verification_ok = certificate_authority::verify_data_signature(
        data_to_verify,
        hex::decode(&req.signature).unwrap(),
        &keypair.keypair_name
    ).unwrap();

    Ok(HttpResponse::Ok().body(format!("{verification_ok}")).into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::PayloadConfig::new(1024*1024*5)) // 5 Mb
            .service(create_keypair)
            .service(upload_file_and_sign)
            .service(upload_file_and_verify)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}


// fn main() {
//     println!("ECDSA test");
//
//     let file_to_sign = Path::new("files/test_file.txt");
//     let sgn_file = Path::new("files/test_file.sgn");
//     let crt_file = Path::new("files/test.crt");
//
//     // let Ok((sgn, crt)) = gen_keypair_and_sign(file_to_sign) else {
//     //     panic!("Signing error");
//     // };
//     //
//     // let mut sgn_file = File::create(sgn_file).unwrap();
//     // let mut crt_file = File::create(crt_file).unwrap();
//     // sgn_file.write_all(sgn.as_ref());
//     // crt_file.write_all(crt.as_ref());
//
//     let result = verify_file_sign(file_to_sign, sgn_file, crt_file).unwrap();
//     print!("Sign verification: {result}");
// }