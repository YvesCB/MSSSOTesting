use dotenv::dotenv;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Header, TokenData, Validation};
use reqwest::get;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct JWK {
    kty: String,
    #[serde(rename = "use")]
    use_field: String,
    kid: String,
    x5t: String,
    n: String,
    e: String,
    x5c: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct JWKS {
    keys: Vec<JWK>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // Add required claims like 'sub', 'exp', etc.
    sub: String,
    exp: usize,
    aud: String,
}

async fn fetch_public_key(jwks_url: &str, kid: &str) -> Option<JWK> {
    let response = get(jwks_url).await.ok()?;
    let jwks: JWKS = response.json().await.ok()?;
    jwks.keys.into_iter().find(|key| key.kid == kid)
}

fn decode_token_header(token: &str) -> Result<Header, jsonwebtoken::errors::Error> {
    let header = decode_header(token)?;
    Ok(header)
}

fn validate_token(
    token: &str,
    public_key: &str,
    audience: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_ref()).expect("Invalid public key");
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[audience]);
    decode::<Claims>(token, &decoding_key, &validation)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = &std::env::var("TOKEN").expect("No TOKEN in .env");
    let jwks_url = &std::env::var("JWKS_URL").expect("No JWKS_URL in .env");
    let audience = &std::env::var("AUD").expect("No AUD in .env");

    // Decode the token header to get the `kid`
    let header = match decode_token_header(token) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Failed to decode token header: {}", e);
            return;
        }
    };

    let kid = header.kid.unwrap();

    // Fetch the public key from JWKS endpoint
    let jwk = match fetch_public_key(jwks_url, &kid).await {
        Some(jwk) => jwk,
        None => {
            eprintln!("Could not get matching public key");
            return;
        }
    };

    let mut key = String::new();
    key.push_str("-----BEGIN CERTIFICATE-----\n");
    key.push_str(jwk.x5c.first().unwrap());
    key.push_str("\n-----END CERTIFICATE-----\n");

    match validate_token(token, &key, audience) {
        Ok(token_data) => println!("Token is valid: {:?}", token_data.claims),
        Err(err) => println!("Invalid token: {}", err),
    }
}
