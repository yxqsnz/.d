use std::error::Error;

pub fn login(name: &str, passwd: &str) -> Result<(), std::io::Error> {
    let service = "system-auth";
    let mut auth = pam::Authenticator::with_password(service).unwrap();
    auth.get_handler().set_credentials(name, passwd);
    if auth.authenticate().is_ok() && auth.open_session().is_ok() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid password",
        ))
    }
}
