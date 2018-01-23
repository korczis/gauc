#[derive(Debug, Clone, Copy)]
pub enum AuthenticatorInternal {}

unsafe impl Send for AuthenticatorInternal {}
unsafe impl Sync for AuthenticatorInternal {}

use super::AuthType;
use super::super::funcs::lcbauth_set_mode;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct AuthenticatorPtr {
    ptr: *mut AuthenticatorInternal
}


impl AuthenticatorPtr {
    pub fn new() -> AuthenticatorPtr {
        unsafe {
            AuthenticatorPtr {
                ptr: ::std::mem::zeroed()
            }
        }
    }

    /*
    pub fn set_mode(&mut self, auth_type: AuthType) {
        unsafe {
            lcbauth_set_mode(self as *mut Authenticator, auth_type);
        }
    }

    pub fn add_pass(&mut self, username: &String, password: &String) {

    }
    */
}
unsafe impl Send for AuthenticatorPtr {}
unsafe impl Sync for AuthenticatorPtr {}

pub type Authenticator = AuthenticatorPtr;
