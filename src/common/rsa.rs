use std::{collections::HashMap, fs, path::Path, sync::LazyLock};

use rsa::{
    pkcs1v15::{Signature, VerifyingKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    sha2::Sha256,
    signature::Verifier,
    traits::PublicKeyParts,
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

static RSAKEY_CONFIG: LazyLock<HashMap<u32, (RsaPrivateKey, RsaPublicKey)>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    fs::read_dir("assert/rsakey_config")
        .unwrap()
        .filter_map(|entry| match entry {
            Ok(entry) => Some(entry),
            Err(_) => None,
        })
        .filter(|entry| match entry.file_type() {
            Ok(file_type) => file_type.is_dir(),
            Err(_) => false,
        })
        .for_each(|entry| {
            let path = entry.path();
            let rsa_ver = path.file_stem().unwrap().to_str().unwrap().parse().unwrap();
            let client_public_key = fs::read_to_string(path.join("client_public_key.pem")).unwrap();
            let server_private_key =
                fs::read_to_string(path.join("server_private_key.pem")).unwrap();
            map.insert(
                rsa_ver,
                (
                    RsaPrivateKey::from_pkcs8_pem(&client_public_key).unwrap(),
                    RsaPublicKey::from_public_key_pem(&server_private_key).unwrap(),
                ),
            );
        });
    map
});

static PASSWORD_KEY: LazyLock<RsaPublicKey> = LazyLock::new(|| {
    RsaPublicKey::from_public_key_pem(
        &fs::read_to_string(Path::new("assert/password_key.pem")).unwrap(),
    )
    .unwrap()
});

pub fn rsa_decrypt(content: &str, rsa_ver: u32) -> Result<Vec<u8>, String> {
    let priv_key = match RSAKEY_CONFIG.get_key_value(&rsa_ver) {
        Some((_, (client_public_key, _))) => client_public_key,
        None => return Err(format!("rsa_ver {} not found", rsa_ver)),
    };

    let raw = match base64::decode(content) {
        Ok(raw) => raw,
        Err(e) => return Err(format!("failed to decode base64: {}", e)),
    };

    let mut decrypted = Vec::new();
    for chunk in raw.chunks(priv_key.size()) {
        let dec_chunk = match priv_key.decrypt(Pkcs1v15Encrypt, &chunk) {
            Ok(decrypted) => decrypted,
            Err(e) => return Err(format!("failed to decrypt: {}", e)),
        };
        decrypted.extend_from_slice(&dec_chunk);
    }

    return Ok(decrypted);
}

pub fn rsa_encrypt(content: &[u8], rsa_ver: u32) -> Result<String, String> {
    let pub_key = match RSAKEY_CONFIG.get_key_value(&rsa_ver) {
        Some((_, (_, server_private_key))) => server_private_key,
        None => return Err(format!("rsa_ver {} not found", rsa_ver)),
    };

    let encrypted =
        match pub_key.encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, content) {
            Ok(encrypted) => encrypted,
            Err(e) => return Err(format!("failed to encrypt: {}", e)),
        };

    return Ok(base64::encode(encrypted));
}

pub fn rsa_verify_sign(content: &[u8], sign: &str, rsa_ver: u32) -> Result<bool, String> {
    let verify_key = match RSAKEY_CONFIG.get_key_value(&rsa_ver) {
        Some((_, (_, server_private_key))) => {
            VerifyingKey::<Sha256>::new(server_private_key.clone())
        }
        None => return Err(format!("rsa_ver {} not found", rsa_ver)),
    };

    let raw: Vec<u8> = match base64::decode(sign) {
        Ok(raw) => raw,
        Err(_) => return Err("failed to decode base64".to_string()),
    };
    let sign = match Signature::try_from(raw.as_slice()) {
        Ok(sign) => sign,
        Err(_) => return Err("failed to convert to signature".to_string()),
    };

    return Ok(verify_key.verify(content, &sign).is_ok());
}

pub fn password_encrypt(password: &str) -> Result<String, String> {
    let encrypted = match PASSWORD_KEY.encrypt(
        &mut rand::thread_rng(),
        Pkcs1v15Encrypt,
        password.as_bytes(),
    ) {
        Ok(encrypted) => encrypted,
        Err(e) => return Err(format!("failed to encrypt: {}", e)),
    };

    return Ok(base64::encode(encrypted));
}
