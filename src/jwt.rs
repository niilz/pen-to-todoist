use crate::auth::{Claims, Creds};
use crate::utils::console_log;
use base64::{CharacterSet::UrlSafe, Config};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const CREDENTIAL_JSON: &str = r#"{
  "type": "service_account",
  "project_id": "hand-to-list",
  "private_key_id": "f9b3c1e160658dd75030464983393d2da403eba5",
  "private_key": "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDwk0W3wf3aNVn1\nKeu3Av2LhV65nzTK6YuXUz75PKKyRxP4Z0HUW+ZV6Nghld04CDjifEE0Cebz7n6Y\ny8Clk0cwWb3VHXuE4jc2NXttU9aM1ui88sVfnyxFXNSeCkdUnOvT/EsiMOQy3jVl\nPiPuD3wI6RgFby8y4N/KZIGbu5FPPS+weSGEuEArEHH0NGzXDXmbYG+yU7fp2hao\ne5w0Cu+G8HJ37oPb9E7QNPUbHI9R+JmbSl0/1j9wucbb4I6IXn+m6QCbCJ4kzwEu\nVlu28icB5SsI2JAPq6ctoTp7kLsDjKr5s8Ye6oCFkizAMk4PKp2garh/KY1BF+X3\nI7yDjjiXAgMBAAECggEACta3x2WoEugLG4PoqMrWXYOEFweDO2PLnuUTQJYTcRt0\nB4xaEj/ptWko2P/a2FFr2VhmdPpqkLGSu+jszwjppitaOtB55izC9rWbeqYnqRoY\nS5sB0CMTEt+cfK+oiGQMLqApSmsQJ503TqO+vqpX7kGCNuHMK0Cnvy30C5jRx9Nl\nhDD2aAuH04jNHAKpR3HqiRDmo1jzrcqbh4tnlETCKSzFIWoFWDSob0sTx6egqzsP\nvbWEbcJWAaXzNLB5wc3WlBkkLxvNaoQWcsJYl2zo3gK9aJnPoPq5Gec6z+0w0kwv\nhPp+dz+bWngs51PsQMgTRgSTWEtdfGo9kQaiPITKkQKBgQD/jA5vynTYK7BgxUIS\nAg8k9AoOQaGA1FhmbI1EHbfAYD1pXCyq8041SI8V3BrOo2ov64UeL8agLCXuU2wR\nMyTFQVAax5m7+I0boxbA9u8Ugv0piwIzq0qM5uf/xN8elYfmMeZrSB0DrRutACa1\nBcSTV3Gy+WDAgJyoIRdyT2YtQwKBgQDxAGxRk0DQ/LhF8WiegfIUHtxUs9aoBS6S\nZUsTKLvwnc3UJZlg883edi001/EFKMVMFWQSNtY/B+PxCv87EaIy6sty9DUYTf+S\nwerAVyWN0h2jN7wZcq70EsopKFR8oDfSnAu2XDNlbfoM4pirETslrLE6+IPrlCv/\niFimpJsIHQKBgQDmmXkRke08gLgpqvdDDs6htwI3+SuG+JdI3d3gQLznATGJmN7J\n6MKDPJW50SPzoe0ZjvtR0ST4tr4HwQm8v50Hzzc84y8sO08CSHoo6Ou3Y7iVu1Xi\nUEj2uwtRd1Mr4x1+MTtPRgTAo35c78z9/1Vy7C6ypWyUabpz0WC6C0IVxwKBgG1q\nXcMjcZfwRNEsau0b3gYYhLvH8jrSL3SznEQUiE8TtfENKPeFeP/480k8iOZovjpu\ns8I5N0fikzBKZ9ovhU5MnxWPndNtBg1hEO1GI3yh2mbR1QpQsPGK3lGVR9ZU/0MV\nKZQfhA4WwiG7dsijyBCwuL4nOe1olhkrk+QEc4ZZAoGANRh3T5jCAYOLA7OG8PI5\nwCSH4UvJxcSxeN4egi82V3NPr3ofJKLMHO0UUG5Q3hZBQK1wgMRQ5gvwEBVoq6UL\nF5f+yMEM4AxH1+uuZ6QwjIkRLaVr03LLyoMeaoIRvgcYqdX88tEQSZ7n3nIiUM3C\niOQgy9Pe0xgRvmzvS8CHiMs=\n-----END PRIVATE KEY-----\n",
  "client_email": "master-dev@hand-to-list.iam.gserviceaccount.com",
  "client_id": "109347730862136476129",
  "auth_uri": "https://accounts.google.com/o/oauth2/auth",
  "token_uri": "https://oauth2.googleapis.com/token",
  "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
  "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/master-dev%40hand-to-list.iam.gserviceaccount.com"
}"#;

#[wasm_bindgen]
pub fn create_jwt() -> Option<String> {
    // Load credentials
    let creds: Creds = serde_json::from_str(CREDENTIAL_JSON).unwrap();

    let header = Header::new();
    console_log("header", &header);
    let header_json = serde_json::to_string(&header).unwrap();
    console_log("header_json", &header_json);
    let claims = Claims::new(&creds.client_email, &creds.token_uri);
    console_log("claims", &claims);
    let claims_json = serde_json::to_string(&claims).unwrap();
    console_log("claims_json", &claims_json);

    let config = Config::new(UrlSafe, false);

    let header_64 = base64::encode_config(header_json, config);
    let claims_64 = base64::encode_config(claims_json, config);

    let jwt_data = format!("{}.{}", header_64, claims_64);
    console_log("header_64.claims_64", &jwt_data);

    let private_key = create_private_key(&creds.private_key);
    console_log("public_key", &private_key);

    let jwt_data_hashed = hmac_sha256::Hash::hash(jwt_data.as_bytes());
    let padding_scheme = rsa::PaddingScheme::new_pkcs1v15_sign(Some(rsa::Hash::SHA2_256));
    let signature = private_key
        .sign(padding_scheme, &jwt_data_hashed)
        .expect("Could not sign jwt-data");
    let signature_64 = base64::encode_config(signature, config);
    console_log("signature_64", &signature_64);

    let jwt = format!("{}.{}.{}", header_64, claims_64, signature_64);
    console_log("jwt", &jwt);
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
