mod error;
mod models;
pub mod session;

pub use error::InternalServerError;
pub use models::KeyData;
pub use models::PasteData;
pub use models::ResponseInfo;
pub use models::UserData;

pub const GUEST_ID: &str = "guest";
