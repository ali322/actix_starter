use actix_web::{HttpResponseBuilder, error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::json;
use std::{collections::HashMap, io, num};
use thiserror::Error;
use validator::ValidationErrors;

use crate::repository::DBError;

#[derive(Debug, Error)]
#[error("{}", .0)]
pub enum APIError {
    IO(#[from] io::Error),
    ParseInt(#[from] num::ParseIntError),
    Validate(#[from] ValidationErrors),
    DB(#[from] DBError),
    Custom(String),
}

impl Serialize for APIError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Validate(e) => {
                let map = format_validator_errors(e);
                serializer.collect_map(map)
            }
            _ => {
                let s = format!("{}", self);
                serializer.collect_str(&s)
            }
        }
    }
}

fn format_validator_errors(e: &ValidationErrors) -> HashMap<String, String> {
    let errors = e
        .field_errors()
        .into_iter()
        .map(|(k, v)| {
            let errors = v
                .iter()
                .map(|e| match &e.message {
                    Some(msg) => msg.to_string(),
                    None => format!("{} is invalid", e.code.to_string()),
                })
                .collect::<String>();
            (k.to_string(), errors)
        })
        .collect::<HashMap<_, _>>();
    errors
}

impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }
    fn error_response(&self) -> HttpResponse {
        let (code, message) = match self {
            _ => (-2, json!(self)),
        };
        let json_body = json!({
          "code": code, "message": message
        });
        HttpResponseBuilder::new(self.status_code()).json(json_body)
    }
}
