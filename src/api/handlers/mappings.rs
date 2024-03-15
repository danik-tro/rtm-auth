use crate::domain::models::{CreateUserDto, DomainError};

use super::registration::SignupRequest;

impl TryFrom<SignupRequest> for CreateUserDto {
    type Error = DomainError;

    fn try_from(value: SignupRequest) -> Result<Self, Self::Error> {
        let user = Self {
            email: value.email.as_str().try_into()?,
            first_name: value.first_name,
            last_name: value.last_name,
            password: value.password,
        };

        Ok(user)
    }
}
