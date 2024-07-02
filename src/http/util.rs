use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, pkcs1::DecodeRsaPrivateKey};

// from https://github.com/thexeondev/ZZZKeys
const DISPATCH_KEY_2: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAxSJVZ+f9t3v4n0h5E05c4pr03KlHUFFKT4HOmvwwmzvKJWTL
itX8f5Et6Q2+t4KFzfeNXoDVVs+NF8k1ULJI0HCrHb46NiZncEWVHzYvd8QOIACR
iMLeLiCwFasnScvN5jGCW7/wSVwk0vizxNSCP2BYyZCO5Nw47BuIJkFmj1leuqIT
ZLntivnbQLeSSnaruZ+o9im+I8k3BpT654CflsV6WnaliWu0JKvEZbvY+x4Yx/nW
oqTWAx8DYClRaz9T7W5v9b55ueMrTo0QGMAfRjutqgpiieLY2Inyv8YEfpIg5keh
Sgti/A4TbKER8RrjHo+j73yOId4FNBGgwUISSQIDAQABAoIBADhXrsdK1AYcJZwI
z6oH7J5LoY5rx1kaUr3UoONFDvNfLFBxtYcRz9/584A3kHnMkUej6EDJW7A7E91n
gZh1zD/e49lVZDfT1B6PlKZeC0UUwOC+qNnIsE3Iw+r9hvYdjNSs89AbVKby8gdm
U6L/BdbtR5McgOrCUnaRC+G7MX/oj4xXVemSdwTp29tJrajZOtFqw2Deali5A0hf
GHhYM4p6y1srmhK1ooS72nPFUD5LuOJaRjAc9nVHv7uK4vcIahsBusCOvzRfxYyp
tTirnVP337OZdLk9DIETomiGbwRWsFvWjqx9as5nz8XkvlKlUNJpe+qW2mlCXQ9S
YZ1v+d0CgYEA6+aZ25vywfMHTXSKaPszMqMoTSo/rnyjgs5AqLFjuDXfz8QtMn1A
2D+fnRstghO8TYUBm2zq3sPd4o/UkwXyx0LiU2YV7jMb+qJWWDTI4nYI8ItEQ3+t
FHf4Jm1AzKHMk6590OqZEDMAA9xVVHUzRdqbHNCASFAQ3+W2vG+rgN8CgYEA1e4q
U+YhRRrheRp7aB3mMvPd3YDNYmtjssCdBOBelwGT3tnRKUSP7LbHay5nokGge581
Efh+0fIurWP0mhHGEACHocxxdLNcWtR8xyNNPsJ8vMWl5z2vCh6AHq25QWQ7LPBn
4bBCZvXm9OAsaXrs9yiB3oyLczvqm5mPPPBvCdcCgYBXLhXWTz/b5ZDhUxyDx7IY
GKwgOGjody+Ge+rxbP1Jar+X7Qw3CIsaYAkA6rZayChjDbCWXB5AzWoG/Ni2rRAY
C+PQpn2cVk8UxUHbv0YuSyjqnM1RlLHGSr/uk5nLBlCwk1/0AX8nuXFb++WdHOMi
AbQpZfTCxLQ3ypFJ+osLSQKBgQCSHlFk6suQotKcX/0oWTuxU6YXs/EUpxr6rdrR
yNpys84nKYlWcZEIcrN0vsZkKgUOSuj5tEwS5CMDpUEdtsQci/AYEEbyGcwb8gSk
E7WgB0EhwrIVZqlW9Qw3aBsf0fElObMKkNRZ9N1iJnfPN/oAuvcxZbed5q/Qxarz
SZneEwKBgHXzoD0VyRcXzabUC7pDDBl9kO6g2GSLu+VBleAD4klTIPCovvlUBXz+
RJ7O7QyD1nVsMpVlvN1Qs1v5sEmk90BIs5uQpUpURVl7fQlhUMA6Q2eh3jaFgsEG
fdu20pNeba5Ied9IcXMVAiQtYq43gdrrNFFq6njrTkpRArZ38UWv
-----END RSA PRIVATE KEY-----";

const DISPATCH_KEY_3: &str = "-----BEGIN RSA PRIVATE KEY-----
MIICXQIBAAKBgQCuRCgK0ZLlhKvPGbz1arcZJH+7DkAkPAaq762RdWJnA4XwfrGA
vso+LY5UBMcKho0UtMAPmeUtKODSkcQCodp+9e3OPgaHy28XYDr/zIHlx5Pc6zOj
1RHl3uGwjnmzFvftBSlAsdijEqzv0WiEnvhnlXYuBJj7Re9sbEwfGHEdNwIDAQAB
AoGAdBCe3r3Aaa9YQsIwsP/XXR6LGAmgvMFh631gi62z0UpubcPj8wyfZJQw5FKe
Qqtk0XKlLH7iPZapTnWZJ+umuqf84RcuilwItH3yTrJw8PeIIxuKE8E+TknKni2I
7SYyKpuuRbW96DC5WhHS4QNRbNwoyCQPfK0WobQItAWuWFkCQQDn0Bj/A0SbnXd8
kNeqefPkPxWvAucAA8+GIoDL5GyM+HUwMW3ZyVNA50vvPVFbCHV8UXzX1shSLEMl
leJ7eX3NAkEAwHLxcB3N2s/GJgco35bEJPQkrv184PXnJRO2cTxCR8ZTrN62oKZb
zD/6j5EcZA6hUou+YjhPLsqcaFJVI2DjEwJACjugbtlpFOl9lc0bCBZ8Eq5KOcCK
C/EfHw+02ctKk2qAWdFExSFjAHF1Y6k0Ry13ajZ6qVh83UvA5w7XwZWaXQJBAIl1
EBI+l3xEj4k17/HTpAGpCFKc6AdeXFzVOaijm8vcUoPVB6izajn3MKY+xYKKGJv3
CGLrSVNokFLLC9ibKVUCQQCKoomFxRojvGmrOLnDYfrWNLGsE6myMsZ5yLYgHsaO
3yCaL021nJLTkUzNU2/09EMO4bO/SGl7OHKrjdUbRAvM
-----END RSA PRIVATE KEY-----";

pub fn decrypt_content(content: &str, rsa_ver: i32) -> String {
    let priv_key = match rsa_ver {
        2 => RsaPrivateKey::from_pkcs1_pem(DISPATCH_KEY_2).expect("failed to parse private key"),
        3 => RsaPrivateKey::from_pkcs1_pem(DISPATCH_KEY_3).expect("failed to parse private key"),
        _ => panic!("unsupported RSA version"),
    };

    let content = base64::decode(content).expect("failed to decode base64");
    let decrypted = priv_key
        .decrypt(Pkcs1v15Encrypt, &content)
        .expect("failed to decrypt content");
    return String::from_utf8(decrypted).expect("failed to convert to string");
}
