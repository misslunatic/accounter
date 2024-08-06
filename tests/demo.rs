use std::collections::HashMap;
use accounter::{AuthError, AuthProvider, AuthSession, AuthUser, Permission};
use accounter::AuthError::InvalidToken;

// Setup...
struct DemoUser<'a> {
    data: &'a DemoProvider,
    id: String,
}

impl<'a> DemoUser<'a> {
    fn pronouns(&self) -> &String {
        &self.data.users[&self.id].pronouns
    }
}

impl<'a> AuthUser for DemoUser<'a> {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

struct DemoSession<'a> {
    user: DemoUser<'a>
}

impl<'a> AuthSession<DemoUser<'_>> for DemoSession<'a> {
    fn user(&mut self) -> &mut DemoUser {
        &mut self.user
    }

    fn has_permission(&self, permission: &Permission) -> bool {
        todo!()
    }

    fn grant_permission(&mut self, permission: &Permission, state: bool) {
        todo!()
    }
}

struct UserData {
    name: String,
    pronouns: String
}

struct DemoProvider {
    users: HashMap<String, UserData>,
    tokens: HashMap<String, String>, // Token, UserId
    incrementor: u32
}

impl DemoProvider {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            tokens: HashMap::new(),
            incrementor: 0
        }
    }
    pub  fn create_user(&mut self, user_id: String, password: String) {
        self.users.insert(user_id.clone(), UserData {
            name: user_id.clone(),
            pronouns: "any/all".to_string()
        });
    }

    // u+p is here for example only, it is not within scope
    pub fn login(&mut self, user_id: String, password: String) -> Result<String, ()> {
        self.incrementor += 1;
        let token = self.incrementor.to_string();
        self.tokens.insert(token.clone(), user_id);
        Ok(token)
    }
}

impl<'a> AuthProvider<DemoUser<'_>, DemoSession<'_>> for DemoProvider {
    type Token = String;

    fn valid_token(&self, token: Self::Token) -> bool {
        self.tokens.get(&token).is_some()
    }

    fn session_from_token(&self, token: Self::Token) -> Result<DemoSession, AuthError> {
        match self.tokens.get(&token) {
            Some(v) => {
                Ok(DemoSession {
                    user: DemoUser {
                        data: &self,
                        id: v.clone()
                    }
                })
            },
            None => Err(InvalidToken)
        }
    }

    fn session_from_id(&self, id: String) -> Result<DemoSession, AuthError> {
        Ok(DemoSession {
            user: DemoUser {
                data: &self,
                id: id.clone()
            }
        })
    }
}

// The actual demo
fn main() -> Result<(), ()> {
    let demo_permission = Permission::new("demo_feature", "read");
    let mut auth = DemoProvider::new();

    auth.create_user("John Doe".to_string(), "password123".to_string());
    // Tokens are temporary keys to get in to a session - `login` creates a new token
    // `?` is being used to ignore Result<> for brevity.
    let mut session = auth.session_from_token(auth.login("John Doe".to_string(), "password123".to_string())?)?;

    assert!(!session.has_permission(&demo_permission));
    session.grant_permission(&demo_permission, true);
    assert!(session.has_permission(&demo_permission));

    let user_id = session.user().id();
    // Add custom functions on your implementation
    let pronouns = session.user().pronouns();

    Ok(())
}