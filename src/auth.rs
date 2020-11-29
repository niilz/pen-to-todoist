use crate::utils::{console_log, fetch};
use jwt_simple::{claims, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit};

pub(crate) const ONE_HOUR_MILLIS: i64 = 60 * 60 * 10000;
const GOOGLE_OAUTH_URL: &str = "https://oauth2.googleapis.com/token";

#[wasm_bindgen]
pub fn auth() -> String {
    "Hi from AUTH".to_string()
}

pub async fn get_access_token(jwt: &str) -> Result<AccessTokenResponse, JsValue> {
    let form = web_sys::FormData::new().unwrap();
    form.append_with_str("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer")
        .unwrap();
    form.append_with_str("assertion", jwt).unwrap();

    let request = init_request(form);
    console_log("auth.rs/get_access_token():", &"Got Token Response");

    let res_json = fetch(request).await?;
    Ok(res_json
        .into_serde::<AccessTokenResponse>()
        .expect("Could not convert into JSON"))
}

fn init_request(form: web_sys::FormData) -> Request {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&form));
    let request =
        Request::new_with_str_and_init(GOOGLE_OAUTH_URL, &opts).expect("Could not create response");
    request
        .headers()
        .set("Content-Type", "multipart/form-data")
        .unwrap();
    request
}
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub(crate) struct Creds {
    pub(crate) r#type: String,
    pub(crate) project_id: String,
    pub(crate) private_key_id: String,
    pub(crate) private_key: String,
    pub(crate) client_email: String,
    pub(crate) client_id: String,
    pub(crate) auth_uri: String,
    pub(crate) token_uri: String,
    pub(crate) auth_provider_x509_cert_url: String,
    pub(crate) client_x509_cert_url: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub(crate) struct Claims {
    pub(crate) iss: String,
    pub(crate) scope: String,
    pub(crate) sub: String,
    pub(crate) aud: String,
    pub(crate) iat: i64,
    pub(crate) exp: i64,
}

impl Claims {
    pub(crate) fn new(email: &str, api_endpoint: &str) -> Self {
        let now = js_sys::Date::now() as i64;
        Claims {
            iss: email.to_string(),
            sub: email.to_string(),
            scope: "https://www.googleapis.com/auth/cloud-platform".to_string(),
            aud: api_endpoint.to_string(),
            iat: now,
            exp: (now + ONE_HOUR_MILLIS),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}
