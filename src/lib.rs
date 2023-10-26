use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tide::{Middleware, Next, Request, Response, StatusCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub uid: String,
    pub exp: u64,
    pub login_time: u64,
}

#[derive(Clone)]
pub struct ApiKeyMiddleware {
    secret_key: String,
}

impl ApiKeyMiddleware {
    pub fn new(secret_key: &str) -> Self {
        Self {
            secret_key: secret_key.to_string(),
        }
    }

    pub fn gen_token(
        &self,
        sub: &str,
        username: &str,
        uid: &str,
        exp: u64,
        login_time: u64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            sub: sub.to_string(),
            username: username.to_string(),
            uid: uid.to_string(),
            exp,
            login_time,
        };
        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret_key.as_bytes()),
        )
    }
}

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for ApiKeyMiddleware {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> tide::Result {
        if let Some(api_key) = req.header("x-api-key") {
            let token_data = decode::<Claims>(
                &api_key.as_str(),
                &DecodingKey::from_secret(self.secret_key.as_bytes()),
                &Validation::default(),
            );

            match token_data {
                Ok(data) => {
                    let claims = data.claims;
                    // TODO: Check sub type
                    req.set_ext(claims);
                    Ok(next.run(req).await)
                }
                Err(_) => {
                    let mut res = Response::new(StatusCode::Unauthorized);
                    res.set_body("Invalid API key");
                    Ok(res)
                }
            }
        } else {
            let mut res = Response::new(StatusCode::Unauthorized);
            res.set_body("API key missing");
            Ok(res)
        }
    }
}
