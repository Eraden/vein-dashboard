use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Session {
    id: Uuid,
    ttl: chrono::NaiveDateTime,
}

struct Inner {
    sessions: Vec<Session>,
}

pub struct SessionManager(Arc<RwLock<Inner>>);

impl SessionManager {
    pub fn new() -> Self {
        let inner = Arc::new(RwLock::new(Inner {
            sessions: Vec::with_capacity(64),
        }));
        let i = inner.clone();
        tokio::spawn(async move {
            let mut timeout = tokio::time::interval(std::time::Duration::from_secs(10));
            loop {
                timeout.tick().await;
                let now = chrono::Utc::now().naive_utc();
                i.write()
                    .await
                    .sessions
                    .extract_if(.., |session| session.ttl <= now)
                    .for_each(|_| ());
            }
        });
        Self(inner)
    }

    pub async fn exists(&self, uuid: Uuid) -> bool {
        let now = chrono::Utc::now().naive_utc();
        self.0
            .read()
            .await
            .sessions
            .iter()
            .any(|session| session.id == uuid && session.ttl > now)
    }

    pub async fn new_session(&self) -> Uuid {
        let uuid = Uuid::new_v4();
        let ttl = chrono::Utc::now() + chrono::Duration::minutes(15);
        self.0.write().await.sessions.push(Session {
            id: uuid,
            ttl: ttl.naive_utc(),
        });
        uuid
    }

    pub async fn refresh(&self, uuid: Uuid) {
        let mut lock = self.0.write().await;
        let Some(session): Option<&mut Session> =
            lock.sessions.iter_mut().find(|session| session.id == uuid)
        else {
            return;
        };
        session.ttl = (chrono::Utc::now() + chrono::Duration::minutes(15)).naive_utc();
    }
}
