use chrono::{Duration, Utc};
use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;

pub fn create_jwt() -> Result<String, jwt::errors::Error> {
    let mut header = jwt::Header::new(jwt::Algorithm::RS256);
    let creds = read_creds_from_file().unwrap();
    header.kid = Some(creds.private_key_id);
    let claims = Claims::new(&creds.client_email, &creds.token_uri);

    let encode_key = jwt::EncodingKey::from_rsa_pem(creds.private_key.as_ref())?;
    let jwt = jwt::encode(&header, &claims, &encode_key)?;

    // println!("jwt: {:#?}", jwt);
    println!("auth.rs/create_jwt(): Created JWT");
    Ok(jwt)
}

fn read_creds_from_file() -> Result<Creds, io::Error> {
    let path_to_cred_file = env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap();
    let creds_string = fs::read_to_string(path_to_cred_file)?;
    let creds = serde_json::from_str::<Creds>(&creds_string)?;
    println!("auth.rs/read_cres_from_file(): Read google-credentials-json");
    Ok(creds)
}

/*
pub async fn get_access_token(
    jwt: String,
    client: &Client,
) -> Result<AccessTokenResponse, reqwest::Error> {
    let form = reqwest::multipart::Form::new()
        .text("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer")
        .text("assertion", jwt);
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .multipart(form)
        .send()
        .await?;
    // println!("Api-res: {:#?}", res);
    println!("auth.rs/get_access_token(): Got Token-Response");
    let res_body = res.json::<AccessTokenResponse>().await;
    // println!("Res-Body-Text: {:#?}", res_body);
    println!("auth.rs/get_access_token(): Parsed Token-Response");
    res_body
}
*/

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
struct Creds {
    r#type: String,
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
struct Claims {
    iss: String,
    scope: String,
    sub: String,
    aud: String,
    iat: i64,
    exp: i64,
}

impl Claims {
    fn new(email: &str, api_endpoint: &str) -> Self {
        let now = Utc::now();
        Claims {
            iss: email.to_string(),
            sub: email.to_string(),
            scope: "https://www.googleapis.com/auth/cloud-platform".to_string(),
            aud: api_endpoint.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(1)).timestamp(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_credentials() {
        let got_creds = match read_creds_from_file() {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Err: {}", e);
                false
            }
        };
        assert_eq!(got_creds, true);
    }
    #[test]
    fn claims_are_correct() {
        let now = Utc::now();
        let expected_iat = now.timestamp();
        let expected_exp = (now + Duration::hours(1)).timestamp();
        let claims = Claims::new("test@mail.com", "https://endpoint.com");
        assert_eq!(claims.iss, "test@mail.com".to_string());
        assert_eq!(claims.sub, "test@mail.com".to_string());
        assert_eq!(claims.aud, "https://endpoint.com".to_string());
        assert_eq!(claims.iat, expected_iat);
        assert_eq!(claims.exp, expected_exp);
    }
    #[test]
    fn creates_jwt() {
        let is_token = match create_jwt() {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Err: {}", e);
                false
            }
        };
        assert_eq!(is_token, true);
    }
    #[test]
    fn validate_jwt_with_decode() {
        use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
        let creds = read_creds_from_file().unwrap();
        let jwt = create_jwt().unwrap();
        let decode_key = match DecodingKey::from_rsa_pem(creds.private_key.as_ref()) {
            Ok(key) => {
                println!("decode-key: {:?}", key);
                Ok(key)
            }
            Err(e) => {
                eprintln!("Err in decode-key: {}", e);
                Err(e)
            }
        };
        let validation = Validation::new(Algorithm::RS256);
        let token_message = decode::<Claims>(&jwt, &decode_key.unwrap(), &validation);
        println!("token_message: {:#?}", token_message);
        assert_eq!(true, true);
    }
}
