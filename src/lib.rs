#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    DataError
}

#[derive(PartialEq)]
pub struct Permission<'a> {
    pub namespace: &'a str,
    pub name: &'a str,
}

impl<'a> Permission<'a> {
    pub fn new(namespace: &'a str, name: &'a str) -> Self {
        Self {
            namespace,
            name
        }
    }
}

pub trait AuthUser {
    type Id;
    fn id(&self) -> Self::Id;
}

pub trait AuthSession<AU: AuthUser> {
    fn user(&mut self) -> &mut AU;
    fn has_permission(&self, permission: &Permission) -> bool;
    fn grant_permission(&mut self, permission: &Permission, state: bool);
}

pub trait AuthProvider<AU: AuthUser, AS: AuthSession<AU>> {
    type Token;
    fn valid_token(&self, token: Self::Token) -> bool;
    fn session_from_token(&self, token: Self::Token) -> Result<AS, AuthError>;
    fn session_from_id(&self, id: AU::Id) -> Result<AS, AuthError>;
}