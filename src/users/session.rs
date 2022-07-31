use actix_identity::Identity;
use actix_session::{Session, SessionInsertError, SessionGetError};
use actix_web::{HttpRequest, dev::Extensions};

// pub fn is_authenticated(session: &Session) -> Result<bool, SessionGetError> {
//     let res = session.get::<bool>("authenticated")?;
//     let val = res.unwrap_or(false);
//     Ok(val)
// }

pub fn create_session(extensions: &Extensions, id: String) -> Result<(), SessionInsertError> {
    Identity::login(extensions, id)?;
    Ok(())
}
