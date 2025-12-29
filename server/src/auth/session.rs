use std::collections::HashMap;

/// I need to be able to create session for users. 
/// 
/// If I have a session that changes, 
///     when the user registers or logs in. The session store is used and is created 
///     When the user disconnects from a game or connects to a game that is the game being dicussed
/// 
/// 
use uuid::Uuid;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::auth::session;
type UserId = Uuid;
type GameId = Uuid;
type SessionId = Uuid;

pub trait SessionStore: Send + Sync{
    fn create(&mut self, user: UserId) -> SessionId;

    fn get_mut(&mut self, session_id: &SessionId) -> Option<&mut Session>;

    fn get(&self, session_id: &SessionId) -> Option<&Session>;
    
    fn delete(&mut self, session: &SessionId) -> Option<Session>;
}

pub struct InMemoryStore {
    sessions: HashMap<SessionId, Session>
}

pub struct Session {
    pub user: UserId,
    pub current_game: Option<GameId>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self { sessions: HashMap::default() }
    }
}

impl SessionStore for InMemoryStore {
    fn create(&mut self, user: UserId) -> SessionId {
        let session_id = SessionId::new_v4();
        self.sessions.insert(
            session_id,
            Session { user, current_game: None }
        );
        session_id
    }

    fn get_mut(&mut self, session_id: &SessionId) -> Option<&mut Session> {
        self.sessions.get_mut(session_id)
    }

    fn get(&self, session_id: &SessionId) -> Option<&Session> {
        self.sessions.get(session_id)
    }

    fn delete(&mut self, session_id: &SessionId) -> Option<Session> {
        self.sessions.remove(session_id)
    }
}


pub static SESSION_STORE: Lazy<RwLock<InMemoryStore>> = Lazy::new(|| RwLock::new(InMemoryStore::new()));