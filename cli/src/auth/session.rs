use serde::{Serialize, Deserialize};
use std::fs;
use uuid::Uuid;
use mockall::automock;

#[derive(Serialize, Deserialize)]
struct SessionFile {
    session_token: String,
}


fn session_file_path() -> std::path::PathBuf {
    let proj = dirs::config_dir().expect("Cannot find session dir");
    proj.join("session.json")
}

pub fn save_session_token(token: &Uuid) -> std::io::Result<()> {
    let path = session_file_path();
    fs::create_dir_all(path.parent().unwrap())?;

    let data = SessionFile {
        session_token: token.to_string(),
    };

    println!("Added session! {:?}", &path);
    fs::write(path, serde_json::to_vec(&data)?)?;
    Ok(())
}

pub fn load_session_token() -> Option<Uuid> {
    let path = session_file_path();
    let bytes = fs::read(path).ok()?;
    let data: SessionFile = serde_json::from_slice(&bytes).ok()?;
    Uuid::parse_str(&data.session_token).ok()
}

pub fn clear_session_token() {
    let _ = fs::remove_file(session_file_path());
}

#[automock]
pub trait SessionKeeper {
    fn save(&self, token: &Uuid) -> std::io::Result<()>;
    fn load(&self) -> Option<Uuid>;
}

struct FileSessionKeeper {}
impl SessionKeeper for  FileSessionKeeper {
    fn save(&self, token: &Uuid) -> std::io::Result<()> {
        let path = session_file_path();
        fs::create_dir_all(path.parent().unwrap())?;

        let data = SessionFile {
            session_token: token.to_string(),
        };

        println!("Added session! {:?}", &path);
        fs::write(path, serde_json::to_vec(&data)?)?;
        Ok(())
    }

    fn load(&self) -> Option<Uuid> {
        let path = session_file_path();
        let bytes = fs::read(path).ok()?;
        let data: SessionFile = serde_json::from_slice(&bytes).ok()?;
        Uuid::parse_str(&data.session_token).ok()
    }
}
