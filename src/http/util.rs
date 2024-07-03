use itertools::Itertools;
use std::collections::HashMap;

use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    traits::PublicKeyParts,
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;

// from https://github.com/thexeondev/ZZZKeys
const DISPATCH_KEY_2: &str = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDFIlVn5/23e/if
SHkTTlzimvTcqUdQUUpPgc6a/DCbO8olZMuK1fx/kS3pDb63goXN941egNVWz40X
yTVQskjQcKsdvjo2JmdwRZUfNi93xA4gAJGIwt4uILAVqydJy83mMYJbv/BJXCTS
+LPE1II/YFjJkI7k3DjsG4gmQWaPWV66ohNkue2K+dtAt5JKdqu5n6j2Kb4jyTcG
lPrngJ+WxXpadqWJa7Qkq8Rlu9j7HhjH+daipNYDHwNgKVFrP1Ptbm/1vnm54ytO
jRAYwB9GO62qCmKJ4tjYifK/xgR+kiDmR6FKC2L8DhNsoRHxGuMej6PvfI4h3gU0
EaDBQhJJAgMBAAECggEAOFeux0rUBhwlnAjPqgfsnkuhjmvHWRpSvdSg40UO818s
UHG1hxHP3/nzgDeQecyRR6PoQMlbsDsT3WeBmHXMP97j2VVkN9PUHo+Upl4LRRTA
4L6o2ciwTcjD6v2G9h2M1Kzz0BtUpvLyB2ZTov8F1u1HkxyA6sJSdpEL4bsxf+iP
jFdV6ZJ3BOnb20mtqNk60WrDYN5qWLkDSF8YeFgzinrLWyuaErWihLvac8VQPku4
4lpGMBz2dUe/u4ri9whqGwG6wI6/NF/FjKm1OKudU/ffs5l0uT0MgROiaIZvBFaw
W9aOrH1qzmfPxeS+UqVQ0ml76pbaaUJdD1JhnW/53QKBgQDr5pnbm/LB8wdNdIpo
+zMyoyhNKj+ufKOCzkCosWO4Nd/PxC0yfUDYP5+dGy2CE7xNhQGbbOrew93ij9ST
BfLHQuJTZhXuMxv6olZYNMjidgjwi0RDf60Ud/gmbUDMocyTrn3Q6pkQMwAD3FVU
dTNF2psc0IBIUBDf5ba8b6uA3wKBgQDV7ipT5iFFGuF5GntoHeYy893dgM1ia2Oy
wJ0E4F6XAZPe2dEpRI/stsdrLmeiQaB7nzUR+H7R8i6tY/SaEcYQAIehzHF0s1xa
1HzHI00+wny8xaXnPa8KHoAerblBZDss8GfhsEJm9eb04Cxpeuz3KIHejItzO+qb
mY888G8J1wKBgFcuFdZPP9vlkOFTHIPHshgYrCA4aOh3L4Z76vFs/Ulqv5ftDDcI
ixpgCQDqtlrIKGMNsJZcHkDNagb82LatEBgL49CmfZxWTxTFQdu/Ri5LKOqczVGU
scZKv+6TmcsGULCTX/QBfye5cVv75Z0c4yIBtCll9MLEtDfKkUn6iwtJAoGBAJIe
UWTqy5Ci0pxf/ShZO7FTphez8RSnGvqt2tHI2nKzzicpiVZxkQhys3S+xmQqBQ5K
6Pm0TBLkIwOlQR22xByL8BgQRvIZzBvyBKQTtaAHQSHCshVmqVb1DDdoGx/R8SU5
swqQ1Fn03WImd883+gC69zFlt53mr9DFqvNJmd4TAoGAdfOgPRXJFxfNptQLukMM
GX2Q7qDYZIu75UGV4APiSVMg8Ki++VQFfP5Ens7tDIPWdWwylWW83VCzW/mwSaT3
QEizm5ClSlRFWXt9CWFQwDpDZ6HeNoWCwQZ927bSk15trkh530hxcxUCJC1irjeB
2us0UWrqeOtOSlECtnfxRa8=
-----END PRIVATE KEY-----";

const DISPATCH_KEY_3: &str = "-----BEGIN PRIVATE KEY-----
MIICdwIBADANBgkqhkiG9w0BAQEFAASCAmEwggJdAgEAAoGBAK5EKArRkuWEq88Z
vPVqtxkkf7sOQCQ8BqrvrZF1YmcDhfB+sYC+yj4tjlQExwqGjRS0wA+Z5S0o4NKR
xAKh2n717c4+BofLbxdgOv/MgeXHk9zrM6PVEeXe4bCOebMW9+0FKUCx2KMSrO/R
aISe+GeVdi4EmPtF72xsTB8YcR03AgMBAAECgYB0EJ7evcBpr1hCwjCw/9ddHosY
CaC8wWHrfWCLrbPRSm5tw+PzDJ9klDDkUp5Cq2TRcqUsfuI9lqlOdZkn66a6p/zh
Fy6KXAi0ffJOsnDw94gjG4oTwT5OScqeLYjtJjIqm65Ftb3oMLlaEdLhA1Fs3CjI
JA98rRahtAi0Ba5YWQJBAOfQGP8DRJudd3yQ16p58+Q/Fa8C5wADz4YigMvkbIz4
dTAxbdnJU0DnS+89UVsIdXxRfNfWyFIsQyWV4nt5fc0CQQDAcvFwHc3az8YmByjf
lsQk9CSu/Xzg9eclE7ZxPEJHxlOs3ragplvMP/qPkRxkDqFSi75iOE8uypxoUlUj
YOMTAkAKO6Bu2WkU6X2VzRsIFnwSrko5wIoL8R8fD7TZy0qTaoBZ0UTFIWMAcXVj
qTRHLXdqNnqpWHzdS8DnDtfBlZpdAkEAiXUQEj6XfESPiTXv8dOkAakIUpzoB15c
XNU5qKOby9xSg9UHqLNqOfcwpj7FgooYm/cIYutJU2iQUssL2JspVQJBAIqiiYXF
GiO8aas4ucNh+tY0sawTqbIyxnnItiAexo7fIJovTbWcktORTM1Tb/T0Qw7hs79I
aXs4cquN1RtEC8w=
-----END PRIVATE KEY-----";

const PASSWORD_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDDvekdPMHN3AYhm/vktJT+YJr7
cI5DcsNKqdsx5DZX0gDuWFuIjzdwButrIYPNmRJ1G8ybDIF7oDW2eEpm5sMbL9zs
9ExXCdvqrn51qELbqj0XxtMTIpaCHFSI50PfPpTFV9Xt/hmyVwokoOXFlAEgCn+Q
CgGs52bFoYMtyi+xEQIDAQAB
-----END PUBLIC KEY-----";

const SIGN_KEY: &[u8] = b"";

pub fn decrypt_content(content: &str, rsa_ver: i32) -> Result<String, String> {
    let priv_key = match rsa_ver {
        2 => RsaPrivateKey::from_pkcs8_pem(DISPATCH_KEY_2).unwrap(),
        3 => RsaPrivateKey::from_pkcs8_pem(DISPATCH_KEY_3).unwrap(),
        _ => return Err("unknown rsa ver".to_string()),
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

    return match String::from_utf8(decrypted) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("failed to convert to utf8: {}", e)),
    };
}

pub fn password_encrypt(password: &str) -> Result<String, String> {
    let pub_key = match RsaPublicKey::from_public_key_pem(PASSWORD_KEY) {
        Ok(key) => key,
        Err(e) => return Err(format!("failed to parse public key: {}", e)),
    };

    let encrypted = match pub_key.encrypt(
        &mut rand::thread_rng(),
        Pkcs1v15Encrypt,
        password.as_bytes(),
    ) {
        Ok(encrypted) => encrypted,
        Err(e) => return Err(format!("failed to encrypt: {}", e)),
    };

    return Ok(base64::encode(encrypted));
}

pub fn sign_data<T>(data: &T) -> String
where
    T: serde::ser::Serialize,
{
    let t = serde_json::to_value(&data).unwrap();
    let t1: HashMap<String, Value> = serde_json::from_value(t).unwrap();
    let t2: HashMap<String, String> = t1
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string(),
                },
            )
        })
        .collect();

    let mut keys = t2.keys().sorted();
    let mut s = match keys.next() {
        Some(key) => format!("{}={}", key, t2[key]),
        None => return String::new(),
    };

    for key in keys {
        s.push_str(format!("&{}={}", key, t2[key]).as_str());
    }

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(SIGN_KEY).unwrap();
    mac.update(s.as_bytes());
    let result = mac.finalize();
    return hex::encode(result.into_bytes());
}
