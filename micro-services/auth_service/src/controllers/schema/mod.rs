use bson::oid::ObjectId;
use serde::{self, Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteByUserId {
    pub id: String,
    pub password: String,
}

impl DeleteByUserId {
    pub fn hash_password(&mut self) {
        self.password =  base16::encode_lower(&self.password);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhoneModel {
    pub prefix: String,
    pub number: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordModel {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: PhoneModel,
    pub address: AddressModel,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailModel {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendAccountModel {
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressModel {
    pub place: String,
    pub city: String,
    pub zip: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserInfo {
    pub id: String,
    pub user_info: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserPassword {
    pub id: String,
    pub set_password: PasswordModel,
}

impl PasswordModel {
    pub fn hash_password(&mut self) {
        self.new_password =  base16::encode_lower(&self.new_password);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    Admin,
    Client,
    ProductOwner,
    Developer,
}

impl FromStr for Role {
    type Err = ();

    fn from_str(input: &str) -> Result<Role, Self::Err> {
        match input {
            "Admin" => Ok(Role::Admin),
            "Client" => Ok(Role::Client),
            "ProductOwner" => Ok(Role::ProductOwner),
            "Developer" => Ok(Role::Developer),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDeserializeModel {
    pub _id: ObjectId,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: PhoneModel,
    pub address: AddressModel,

    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseModel {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: PhoneModel,
    pub address: AddressModel,

    pub role: Role,
}

impl UserResponseModel {
    pub fn build_user(user: UserDeserializeModel) -> UserResponseModel {
        UserResponseModel {
            id: user._id.to_string(),
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            address: user.address,
 
            role: user.role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: PhoneModel,
    pub address: AddressModel,

    pub role: Role,
}

impl UserModel {
    pub fn hash_password(&mut self) {
        self.password =  base16::encode_lower(&self.password);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub id: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginModel {
    pub email: String,
    pub password: String,
}

impl UserLoginModel {
    pub fn hash_password(&mut self) {
        self.password =  base16::encode_lower(&self.password);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponseModel {
    pub user: UserResponseModel,
    pub token: String,
}
