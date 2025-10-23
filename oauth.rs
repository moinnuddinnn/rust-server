use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use std::env;

#[get("/auth/google/login")]
pub async fn google_login() -> impl Responder {
    let client_id = env::var("GOOGLE_CLIENT_ID").unwrap();
    let redirect_uri = env::var("GOOGLE_REDIRECT_URI").unwrap();
    let scope = "https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/userinfo.profile";
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline",
        client_id, redirect_uri, scope
    );
    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[derive(Deserialize)]
pub struct GoogleAuthRequest {
    code: String,
}

#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[get("/auth/google/callback")]
pub async fn google_callback(query: web::Query<GoogleAuthRequest>) -> impl Responder {
    let client = reqwest::Client::new();
    let params = [
        ("code", query.code.clone()),
        ("client_id", env::var("GOOGLE_CLIENT_ID").unwrap()),
        ("client_secret", env::var("GOOGLE_CLIENT_SECRET").unwrap()),
        ("redirect_uri", env::var("GOOGLE_REDIRECT_URI").unwrap()),
        ("grant_type", "authorization_code".to_string()),
    ];

    // Exchange code for access token
    let token_response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .unwrap()
        .json::<GoogleTokenResponse>()
        .await
        .unwrap();

    // Fetch user info
    let user_info = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(&token_response.access_token)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("Google user info: {}", user_info))
}
