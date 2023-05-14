use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthError {
    has_error: bool,
    errors: AuthErrorDetails,
    error: String,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for AuthError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        &self.error
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl AuthError {
    pub fn has_error(&self) -> &bool {
        &self.has_error
    }
    pub fn errors(&self) -> &AuthErrorDetails {
        &self.errors
    }
    pub fn error(&self) -> &str {
        &self.error
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct System {
    message: String,
    code: u64,
}
impl System {
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn code(&self) -> &u64 {
        &self.code
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthErrorDetails {
    description: Option<String>,
    system: System,
}
impl AuthErrorDetails {
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn system(&self) -> &System {
        &self.system
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiError {
    error: ApiErrorDetails,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for ApiError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        if self.error().user_message().len() > 0 {
            self.error().user_message()
        } else if self.error().message().len() > 0 {
            self.error().message()
        } else if self.error().reason().len() > 0 {
            self.error().reason()
        } else {
            ""
        }
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl ApiError {
    pub fn error(&self) -> &ApiErrorDetails {
        &self.error
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMessageDetail {
    description: Option<String>,
}
impl UserMessageDetail {
    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiErrorDetails {
    user_message: String,
    message: String,
    reason: String,
    user_message_details: UserMessageDetail,
}
impl ApiErrorDetails {
    pub fn user_message(&self) -> &str {
        &self.user_message
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
    pub fn user_message_details(&self) -> &UserMessageDetail {
        &self.user_message_details
    }
}
