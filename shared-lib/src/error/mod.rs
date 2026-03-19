mod server_response;

pub use server_response::*;

pub type AppServerResult<T = ()> = Result<T, ServerErrorResponse>;
