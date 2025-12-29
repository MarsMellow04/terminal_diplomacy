use cli::commands::register::RegisterCommand;
use mockall::predicate::*;
use uuid::{Uuid, uuid};

use cli::commands::{
    create::CreateCommand,
    join::JoinCommand,
    login::LoginCommand,
};
use cli::commands::util::{Client, CommandError};
use cli::auth::session::SessionKeeper;
use cli::commands::util::MockClient;
use cli::auth::session::MockSessionKeeper;
// --------------------------------------------------
// CREATE COMMAND TESTS
// --------------------------------------------------

#[test]
fn create_sends_correct_message() {
    let token = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

    let mut client = MockClient::new();
    client
        .expect_send()
        .with(eq(format!("CREATE;{}\n", token)))
        .times(1)
        .returning(|_| Ok(()));

    let mut session = MockSessionKeeper::new();
    session
        .expect_load()
        .times(1)
        .returning(move || Some(token));

    let mut cmd = CreateCommand::new(client, session);
    cmd.execute().unwrap();
}

#[test]
fn create_fails_without_session_token() {
    let client = MockClient::new();

    let mut session = MockSessionKeeper::new();
    session
        .expect_load()
        .times(1)
        .returning(|| None);

    let mut cmd = CreateCommand::new(client, session);
    let err = cmd.execute().unwrap_err();

    assert!(matches!(err, CommandError::NoSessionToken));
}

#[test]
fn create_fails_when_client_send_fails() {
    let token = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

    let mut client = MockClient::new();
    client
        .expect_send()
        .times(1)
        .returning(|_| Err(CommandError::WriteFailure));

    let mut session = MockSessionKeeper::new();
    session
        .expect_load()
        .returning(move || Some(token));

    let mut cmd = CreateCommand::new(client, session);
    let err = cmd.execute().unwrap_err();

    assert!(matches!(err, CommandError::WriteFailure));
}

// --------------------------------------------------
// JOIN COMMAND TESTS
// --------------------------------------------------

#[test]
fn join_sends_correct_message() {
    let token = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    let game_id = "game_1";

    let mut client = MockClient::new();
    client
        .expect_send()
        .with(eq(format!("JOIN;{};{}\n", token, game_id)))
        .times(1)
        .returning(|_| Ok(()));

    let mut session = MockSessionKeeper::new();
    session
        .expect_load()
        .times(1)
        .returning(move || Some(token));

    let mut cmd = JoinCommand::new(
        client,
        session,
        game_id.to_string(),
    );

    cmd.execute().unwrap();
}

// --------------------------------------------------
// LOGIN COMMAND TESTS
// --------------------------------------------------

#[test]
fn login_sends_message_and_saves_session() {
    let token = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

    // ---- client ----
    let mut client = MockClient::new();

    client
        .expect_send()
        .with(eq("LOGIN;user_1;password_1\n"))
        .times(1)
        .returning(|_| Ok(()));

    client
        .expect_read()
        .times(1)
        .returning(move || Ok(token.to_string()));

    // ---- session ----
    let mut session = MockSessionKeeper::new();
    session
        .expect_save()
        .with(eq(token))
        .times(1)
        .returning(|_| Ok(()));

    let mut cmd = LoginCommand::new(
        client,
        session,
        "user_1".to_string(),
        "password_1".to_string(),
    );

    cmd.execute().unwrap();
}

#[test]
fn login_fails_when_read_fails() {
    let mut client = MockClient::new();

    client
        .expect_send()
        .returning(|_| Ok(()));
    client
        .expect_read()
        .times(1)
        .returning(|| Err(CommandError::NoSessionTokenRead));

    let mut session = MockSessionKeeper::new();
    session.expect_save().times(0);

    let mut cmd = LoginCommand::new(
        client,
        session,
        "user".to_string(),
        "pass".to_string(),
    );

    let err = cmd.execute().unwrap_err();
    assert!(matches!(err, CommandError::NoSessionTokenRead));
}

#[test]
fn login_fails_on_invalid_uuid() {
    let mut client = MockClient::new();

    client
        .expect_send()
        .returning(|_| Ok(()));
    client
        .expect_read()
        .times(1)
        .returning(|| Ok("not-a-uuid".to_string()));

    let mut session = MockSessionKeeper::new();
    session.expect_save().times(0);

    let mut cmd = LoginCommand::new(
        client,
        session,
        "user".to_string(),
        "pass".to_string(),
    );

    let err = cmd.execute().unwrap_err();
    assert!(matches!(err, CommandError::NoSessionToken));
}

#[test]
fn register_sends_message_and_saves_session() {
    let token = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");

    // ---- client ----
    let mut client = MockClient::new();

    client
        .expect_send()
        .with(eq("REGISTER;user_1;password_1\n"))
        .times(1)
        .returning(|_| Ok(()));

    client
        .expect_read()
        .times(1)
        .returning(move || Ok(token.to_string()));

    // ---- session ----
    let mut session = MockSessionKeeper::new();
    session
        .expect_save()
        .with(eq(token))
        .times(1)
        .returning(|_| Ok(()));

    let mut cmd = RegisterCommand::new(
        client,
        session,
        "user_1".to_string(),
        "password_1".to_string(),
    );

    cmd.execute().unwrap();
}