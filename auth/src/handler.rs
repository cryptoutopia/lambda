use std::fmt;
use std::io::Error;
use lambda_runtime::LambdaEvent;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestAction {
    Login,
    Logout,
    SignUp,
    RefreshToken,
    VerifyToken,
    ResetPassword,
}

impl RequestAction {
    pub fn from_str(action: &str) -> Option<Self> {
        match action {
            "login" => Some(RequestAction::Login),
            "logout" => Some(RequestAction::Logout),
            "register" => Some(RequestAction::SignUp),
            "refresh_token" => Some(RequestAction::RefreshToken),
            "verify_token" => Some(RequestAction::VerifyToken),
            "reset_password" => Some(RequestAction::ResetPassword),
            _ => None,
        }
    }
}

impl fmt::Display for RequestAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestAction::Login => write!(f, "login"),
            RequestAction::Logout => write!(f, "logout"),
            RequestAction::SignUp => write!(f, "register"),
            RequestAction::RefreshToken => write!(f, "refresh_token"),
            RequestAction::VerifyToken => write!(f, "verify_token"),
            RequestAction::ResetPassword => write!(f, "reset_password"),
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    token: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    token: String,
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    email: String,
    new_password: String,
}

#[derive(Serialize)]
pub struct Response {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
}

impl Response {
    pub fn new(success: bool, message: String, token: Option<String>) -> Self {
        Response {
            success,
            message,
            token,
        }
    }
}

pub async fn handle_request(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    match RequestAction::from_str(event["action"].as_str().unwrap_or("")) {
        Some(RequestAction::Login) => {
            let login_request: LoginRequest = serde_json::from_value(event.clone()).map_err(|e| {})?;
            let token = login(&login_request.email, &login_request.password).await?;
            Ok(json!(Response::new(true, "Logged in successfully".to_string(), Some(token))))
        }
        Some(RequestAction::Logout) => {
            let logout_request: LogoutRequest = serde_json::from_value(event.clone()).unwrap_or_else(|_| LogoutRequest {
                token: "".to_string(),
            });
            logout(&logout_request.token).await?;
            Ok(json!(Response::new(true, "Logged out successfully".to_string(), None)))
        }
        Some(RequestAction::SignUp) => {
            let register_request: RegisterRequest = serde_json::from_value(event.clone()).unwrap_or_else(|_| RegisterRequest {
                email: "".to_string(),
                password: "".to_string(),
            });
            register(&register_request.email, &register_request.password).await?;
            Ok(json!(Response::new(true, "Registered successfully".to_string(), None)))
        }
        Some(RequestAction::RefreshToken) => {
            let refresh_token_request: RefreshTokenRequest = serde_json::from_value(event.clone()).unwrap_or_else(|_| RefreshTokenRequest {
                token: "".to_string(),
            });
            let new_token = refresh_token(&refresh_token_request.token).await?;
            Ok(json!(Response::new(true, "Token refreshed successfully".to_string(), Some(new_token))))
        }
        Some(RequestAction::VerifyToken) => {
            let verify_token_request: VerifyTokenRequest = serde_json::from_value(event.clone()).unwrap_or_else(|_| VerifyTokenRequest {
                token: "".to_string(),
            });
            let is_valid = verify_token(&verify_token_request.token).await?;
            Ok(json!(Response::new(true, format!("Token is {}", if is_valid { "valid" } else { "invalid" }), None)))
        }
        Some(RequestAction::ResetPassword) => {
            let reset_password_request: ResetPasswordRequest = serde_json::from_value(event.clone()).unwrap_or_else(|_| ResetPasswordRequest {
                email: "".to_string(),
                new_password: "".to_string(),
            });
            reset_password(&reset_password_request.email, &reset_password_request.new_password).await?;
            Ok(json!(Response::new(true, "Password reset successfully".to_string(), None)))
        }
        _ => Ok(json!(Response::new(false, "Invalid action".to_string(), None))),
    }
}