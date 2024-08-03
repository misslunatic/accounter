pub enum AuthError {
    InvalidToken
}

pub struct Permission {
    pub namespace: String,
    pub name: String,
}

impl Permission {
    fn new(namespace: String, name: String) -> Self {
        Self {
            namespace,
            name
        }
    }
}

pub trait AuthUser {
    type Id;
    fn id(&self) -> Self::UserId;
    fn delete(mut self) -> Self::Id;
}

pub trait AuthSession<AU: AuthUser> {
    fn user(&self) -> AU;
    fn has_permission(&self, permission: &Permission) -> bool;
    fn grant_permission(&self, permission: &Permission, state: bool);
}

pub trait AuthProvider<AU: AuthUser, AS: AuthSession<AU>> {
    type Token;
    fn valid_token(&self, token: Self::Token) -> bool;
    fn session_from_token(&self, token: Self::Token) -> Result<AS, AuthError>;
    fn session_from_id(id: AU::Id) -> Result<AS, AuthError>;
}