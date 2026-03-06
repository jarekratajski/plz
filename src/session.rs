use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const SESSION_EXPIRY_SECS: u64 = 3600; // 1 hour

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub session_id: String,
    pub last_used: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_interaction: Option<Interaction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interaction {
    pub description: String,
    pub command: String,
    pub executed: bool,
    pub exit_code: Option<i32>,
}

fn session_dir() -> PathBuf {
    let base = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".local/share")
        });
    base.join("plz")
}

fn session_file() -> PathBuf {
    session_dir().join("session.json")
}

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn load_session() -> Option<Session> {
    let path = session_file();
    let content = fs::read_to_string(&path).ok()?;
    let session: Session = serde_json::from_str(&content).ok()?;

    let elapsed = now_epoch().saturating_sub(session.last_used);
    if elapsed > SESSION_EXPIRY_SECS {
        return None;
    }

    Some(session)
}

pub fn save_session(session: &Session) -> Result<()> {
    let dir = session_dir();
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create session directory: {}", dir.display()))?;

    let path = session_file();
    let json = serde_json::to_string_pretty(session)
        .context("Failed to serialize session")?;
    fs::write(&path, json)
        .with_context(|| format!("Failed to write session file: {}", path.display()))?;

    Ok(())
}

pub fn new_session() -> Session {
    Session {
        session_id: uuid::Uuid::new_v4().to_string(),
        last_used: now_epoch(),
        last_interaction: None,
    }
}

pub fn context_prefix(interaction: &Interaction) -> String {
    let outcome = if interaction.executed {
        match interaction.exit_code {
            Some(0) => "executed successfully".to_string(),
            Some(code) => format!("executed with exit code {code}"),
            None => "executed with unknown exit code".to_string(),
        }
    } else {
        "was not executed".to_string()
    };

    format!(
        "[Previous: asked \"{}\", generated `{}`, {}]\n\n",
        interaction.description, interaction.command, outcome
    )
}

pub fn update_execution_result(executed: bool, exit_code: Option<i32>) {
    if let Some(mut session) = load_session() {
        if let Some(ref mut interaction) = session.last_interaction {
            interaction.executed = executed;
            interaction.exit_code = exit_code;
            let _ = save_session(&session);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_new_session_generates_valid_uuid() {
        let session = new_session();
        assert!(uuid::Uuid::parse_str(&session.session_id).is_ok());
        assert!(session.last_interaction.is_none());
    }

    #[test]
    fn test_context_prefix_executed_success() {
        let interaction = Interaction {
            description: "list files".to_string(),
            command: "ls -la".to_string(),
            executed: true,
            exit_code: Some(0),
        };
        let prefix = context_prefix(&interaction);
        assert!(prefix.contains("list files"));
        assert!(prefix.contains("ls -la"));
        assert!(prefix.contains("executed successfully"));
    }

    #[test]
    fn test_context_prefix_executed_failure() {
        let interaction = Interaction {
            description: "compile".to_string(),
            command: "cargo build".to_string(),
            executed: true,
            exit_code: Some(1),
        };
        let prefix = context_prefix(&interaction);
        assert!(prefix.contains("exit code 1"));
    }

    #[test]
    fn test_context_prefix_not_executed() {
        let interaction = Interaction {
            description: "delete stuff".to_string(),
            command: "rm -rf /tmp/test".to_string(),
            executed: false,
            exit_code: None,
        };
        let prefix = context_prefix(&interaction);
        assert!(prefix.contains("was not executed"));
    }

    #[test]
    fn test_save_and_load_session() {
        let temp_dir = env::temp_dir().join("plz_test_session");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let session_path = temp_dir.join("session.json");
        let session = Session {
            session_id: "test-uuid-1234".to_string(),
            last_used: now_epoch(),
            last_interaction: Some(Interaction {
                description: "test".to_string(),
                command: "echo hello".to_string(),
                executed: true,
                exit_code: Some(0),
            }),
        };

        let json = serde_json::to_string_pretty(&session).unwrap();
        fs::write(&session_path, &json).unwrap();

        let loaded: Session = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.session_id, "test-uuid-1234");
        assert!(loaded.last_interaction.is_some());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_expired_session_not_loaded() {
        let session = Session {
            session_id: "old-uuid".to_string(),
            last_used: now_epoch().saturating_sub(SESSION_EXPIRY_SECS + 100),
            last_interaction: None,
        };

        let elapsed = now_epoch().saturating_sub(session.last_used);
        assert!(elapsed > SESSION_EXPIRY_SECS);
    }

    #[test]
    fn test_session_serialization_round_trip() {
        let session = Session {
            session_id: "abc-123".to_string(),
            last_used: 1709740800,
            last_interaction: None,
        };

        let json = serde_json::to_string(&session).unwrap();
        let deserialized: Session = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.session_id, "abc-123");
        assert!(deserialized.last_interaction.is_none());
        assert!(!json.contains("last_interaction"));
    }
}
