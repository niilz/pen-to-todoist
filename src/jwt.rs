use crate::auth::{Claims, Creds};
use crate::utils::console_log;
use base64::{CharacterSet::UrlSafe, Config};
use serde::{Deserialize, Serialize};

pub(crate) fn create_jwt(credentials_json: &str) -> Option<String> {
    // Load credentials
    let creds: Creds = serde_json::from_str(credentials_json).unwrap();

    let header = Header::new();
    let header_json = serde_json::to_string(&header).unwrap();
    let claims = Claims::new(&creds.client_email, &creds.token_uri);
    let claims_json = serde_json::to_string(&claims).unwrap();
    console_log("WASM - jwt.rs", &"created JWT-header and claims");

    let config = Config::new(UrlSafe, false);

    let header_64 = base64::encode_config(header_json, config);
    let claims_64 = base64::encode_config(claims_json, config);
    console_log("WASM - jwt.rs", &"base64-encoded JWT-header and claims");

    let jwt_data = format!("{}.{}", header_64, claims_64);
    console_log("WASM - jwt.rs", &"connected header_64.claims_64");

    let private_key = create_private_key(&creds.private_key);
    console_log("WASM - jwt.rs", &"created private key");

    let jwt_data_hashed = hmac_sha256::Hash::hash(jwt_data.as_bytes());
    let padding_scheme = rsa::PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256));
    let signature = private_key
        .sign(padding_scheme, &jwt_data_hashed)
        .expect("Could not sign jwt-data");
    let signature_64 = base64::encode_config(signature, config);
    console_log("WASM - jwt.rs", &"created-signature");

    let jwt = format!("{}.{}.{}", header_64, claims_64, signature_64);
    console_log("WASM - jwt.rs", &"Finalized JWT");
    Some(jwt)
}

fn create_private_key(private_key: &str) -> rsa::RSAPrivateKey {
    let pk_trimmed: String = private_key
        .lines()
        .filter(|line| !line.starts_with('-'))
        .collect();
    let pk_64 = base64::decode(pk_trimmed).expect("Coulnd not decode private_key");
    rsa::RSAPrivateKey::from_pkcs8(&pk_64).expect("Could not parse pk_bytes")
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    alg: String,
    typ: String,
}

impl Header {
    fn new() -> Self {
        Header {
            alg: "RS256".to_string(),
            typ: "JWT".to_string(),
        }
    }
}
