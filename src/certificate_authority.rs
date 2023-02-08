#![feature(let_else)]

use ring::{
    error::Unspecified,
    pkcs8, rand,
    signature::{self, KeyPair, Signature, UnparsedPublicKey},
};
use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

// const WORDS: &'static str = "hello rust!";
const KEYSTORE_PATH: &'static str = "key_store/";


fn gen_keypair_bytes() -> pkcs8::Document {
    // Generate a key pair in PKCS#8 (v2) format.
    let rng = rand::SystemRandom::new();
    signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap()
}

fn get_keypair<B: AsRef<[u8]>>(pkcs8_bytes: B) -> signature::Ed25519KeyPair {
    signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap()
}

fn sign_data<B: AsRef<[u8]>>(data: B, key_pair: &signature::Ed25519KeyPair) -> Signature {
    key_pair.sign(data.as_ref())
}

fn verify_signature(
    data: impl AsRef<[u8]>,
    public_key_bytes: impl AsRef<[u8]>,
    sig: impl AsRef<[u8]>,
) -> bool {
    let public_key = UnparsedPublicKey::new(&signature::ED25519, public_key_bytes);

    match public_key.verify(data.as_ref(), sig.as_ref()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn gen_keypair(key_pair_file_name: &str) {
    let key_store_dir = Path::new(KEYSTORE_PATH);

    // Сгенерировать ключи и сохранить
    let pkcs8_bytes = gen_keypair_bytes();
    let mut keypair_file = File::create(key_store_dir.join(key_pair_file_name)).unwrap();
    keypair_file.write_all(pkcs8_bytes.as_ref());

    // let keypair = get_keypair(pkcs8_bytes);
}

pub fn sign_data_with_key<B: AsRef<[u8]>>(data_to_sign: B, key_pair_file_name: &str) -> Result<(Vec<u8>), Error> {
    let keypair_file_path = Path::new(KEYSTORE_PATH).join(key_pair_file_name);

    let pkcs8_bytes = fs::read(keypair_file_path)?;
    let keypair = get_keypair(pkcs8_bytes);

    // Прочитать файл
    // let file_content = fs::read_to_string(file_to_sign).expect("Reading error");
    // println!("file content:\n{file_content}");
    //
    // let file_content = fs::read(file_to_sign).expect("Reading error");

    // Подписать
    let signature = keypair.sign(data_to_sign.as_ref());
    println!("signature:\n{:?}", signature.as_ref());

    // Ok((signature.as_ref().into(), keypair.public_key().as_ref().into()))
    Ok(signature.as_ref().into())
}

pub fn verify_file_sign(signed_file: &Path, sgn_file_path: &Path,
                    crt_file_path: &Path) -> Result<bool, Error> {
    let file_content = fs::read(signed_file).expect("Reading error");

    // Прочитать сертификат и подпись
    let pkcs8_bytes = fs::read(crt_file_path)?;
    let signature = fs::read(sgn_file_path)?;

    Ok(verify_signature(file_content, pkcs8_bytes, signature))
}

pub fn verify_data_signature(data_to_verify: impl AsRef<[u8]>, signature_bytes: impl AsRef<[u8]>,
                                             key_pair_file_name: &str) -> Result<bool, Error> {
    let keypair_file_path = Path::new(KEYSTORE_PATH).join(key_pair_file_name);
    let pkcs8_bytes = fs::read(keypair_file_path)?;
    let keypair = get_keypair(pkcs8_bytes);

    // keypair.public_key().as_ref();

    Ok(verify_signature(data_to_verify, keypair.public_key().as_ref(), signature_bytes))
}