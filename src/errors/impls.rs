use async_graphql::{ ErrorExtensions, Value };
use serde_json::json;

use crate::Errors;

impl Errors {
    /// Set up bad request errors
    pub fn bad_request<T>(errors: T) -> async_graphql::Error
        where T: serde::Serialize
    {
        let value = Value::from_json(json!(errors)).unwrap_or_default();

        Errors::BadRequest.extend_with(|_, e| e.set("errors", value))
    }

    /// Set up bad request error message
    pub fn bad_request_message<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::BadRequest.extend_with(|_, e| e.set("error", error.to_string()))
    }

    /// Set up unauthorized errors
    pub fn unauthorized<T>(errors: T) -> async_graphql::Error
        where T: serde::Serialize
    {
        let value = Value::from_json(
            serde_json::to_value(errors).unwrap_or_default()
        ).unwrap_or_default();

        Errors::Unauthorized.extend_with(|_, e| e.set("errors", value))
    }

    /// Set up unauthorized error message
    pub fn unauthorized_message<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::Unauthorized.extend_with(|_, e| e.set("error", error.to_string()))
    }

    /// Set up payment required errors
    pub fn payment_required<T>(errors: T) -> async_graphql::Error
        where T: serde::Serialize
    {
        let value = Value::from_json(
            serde_json::to_value(errors).unwrap_or_default()
        ).unwrap_or_default();

        Errors::PaymentRequired.extend_with(|_, e| e.set("errors", value))
    }

    /// Set up payment required error message
    pub fn payment_required_message<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::PaymentRequired.extend_with(|_, e| e.set("error", error.to_string()))
    }

    /// Set up forbidden errors
    pub fn forbidden<T>(errors: T) -> async_graphql::Error
        where T: serde::Serialize
    {
        let value = Value::from_json(
            serde_json::to_value(errors).unwrap_or_default()
        ).unwrap_or_default();

        Errors::Forbidden.extend_with(|_, e| e.set("errors", value))
    }

    /// Set up forbidden error message
    pub fn forbidden_message<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::Forbidden.extend_with(|_, e| e.set("error", error.to_string()))
    }

    /// Set up not found errors
    pub fn not_found() -> async_graphql::Error {
        Errors::NotFound.extend()
    }

    /// Set up forbidden errors
    pub fn internal_server_error<T, U>(errors: T, reason: U) -> async_graphql::Error
        where T: serde::Serialize,
              U: ToString
    {
        let value = Value::from_json(
            serde_json::to_value(errors).unwrap_or_default()
        ).unwrap_or_default();

        Errors::InternalServerError(reason.to_string()).extend_with(|_, e| e.set("errors", value))
    }

    /// Set up unauthorized error message
    pub fn internal_server_error_message<T>(error: T) -> async_graphql::Error
        where T: ToString
    {
        Errors::InternalServerError(error.to_string()).extend()
    }

    /// Set up error without extension errors
    pub fn without_extension() -> async_graphql::Error {
        Errors::ErrorWithoutExtensions.extend()
    }
}