use std::collections::HashMap;
use accounter::{AuthError, AuthProvider, AuthSession, AuthUser, Permission};
use accounter::AuthError::InvalidToken;

#[derive(Clone)]
struct UserData {
    name: String,
    pronouns: String
}

// Setup...
struct DemoUser {
    id: String,
    data: UserData
}

impl DemoUser {
    fn pronouns(&self) -> &String {
        &self.data.pronouns
    }
}

impl AuthUser for DemoUser {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

struct DemoSession {
    user: DemoUser
}

impl AuthSession<DemoUser> for DemoSession {
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

impl AuthProvider<DemoUser, DemoSession> for DemoProvider {
    type Token = String;

    fn valid_token(&self, token: Self::Token) -> bool {
        self.tokens.get(&token).is_some()
    }

    fn session_from_token(&self, token: Self::Token) -> Result<DemoSession, AuthError> {
        match self.tokens.get(&token) {
            Some(v) => {
                match self.users.get(v) {
                    Some(data) => {
                        Ok(DemoSession {
                            user: DemoUser {
                                data: data.clone(),
                                id: v.clone()
                            }
                        })
                    }
                    None => Err(AuthError::DataError),
                }
            },
            None => Err(InvalidToken)
        }
    }

    fn session_from_id(&self, id: String) -> Result<DemoSession, AuthError> {
        match self.users.get(&id) {
            Some(data) => {
                Ok(DemoSession {
                    user: DemoUser {
                        data: data.clone(),
                        id: id.clone()
                    }
                })
            }
            None => Err(AuthError::DataError),
        }
    }
}

// The actual demo
fn main() -> Result<(), ()> {
    let demo_permission = Permission::new("demo_feature", "read");
    let mut auth = DemoProvider::new();

    auth.create_user("John Doe".to_string(), "password123".to_string());

    // Tokens are temporary keys to get in to a session - `login` creates a new token
    // `?` is being used to ignore Result<> for brevity.
    let token = auth.login("John Doe".to_string(), "password123".to_string()).expect("err getting token");

    let mut session = auth.session_from_token(token).expect("err getting session");

    assert!(!session.has_permission(&demo_permission));

    session.grant_permission(&demo_permission, true);

    assert!(session.has_permission(&demo_permission));

    let user_id = session.user().id();

    // Add custom functions on your implementation
    let pronouns = session.user().pronouns();

    Ok(())
}