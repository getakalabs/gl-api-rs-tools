pub mod impls;

use async_graphql::ErrorExtensions;

/// Custom error extension
#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("Bad Request")]
    BadRequest,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Payment Required")]
    PaymentRequired,

    #[error("Forbidden")]
    Forbidden,

    #[error("Resource Not Found")]
    NotFound,

    #[error("Internal Server Error")]
    InternalServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}

/// Implement error extension
impl ErrorExtensions for Errors {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_, e|
            match self {
                Self::BadRequest => e.set("code", 400),
                Self::Unauthorized => e.set("code", 401),
                Self::PaymentRequired => e.set("code", 402),
                Self::Forbidden => e.set("code", 403),
                Self::NotFound => e.set("code", 404),
                Self::InternalServerError(reason) => {
                    e.set("code", 500);

                    if !reason.is_empty() {
                        e.set("reason", reason.clone());
                    }
                },
                Self::ErrorWithoutExtensions => {}
            })
    }
}