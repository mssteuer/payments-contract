use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    PermissionDenied = 1,
    BalanceInsufficient = 2,
    WrongToken = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
