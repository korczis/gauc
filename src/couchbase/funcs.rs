use libc::{c_char, c_void};

use super::types::authenticator::Authenticator;
use super::types::auth_type::{AuthType, AuthFlags};
use super::types::callback_type::CallbackType;
use super::types::cmd;
use super::types::create_st::CreateSt;
use super::types::error_type::ErrorType;
use super::types::instance::Instance;
use super::types::response;

pub type ResponseCallback = unsafe extern "C" fn(instance: Instance, cbtype: CallbackType, resp: *const response::Base);

#[link(name = "couchbase")]
extern {
    pub fn lcbauth_add_pass(authenticator: Authenticator, user: *const c_char, pass: *const c_char, flags: AuthFlags) -> ErrorType;
    pub fn lcbauth_new() -> Authenticator;
    pub fn lcb_set_auth(instance: Instance, authenticator: Authenticator);
    pub fn lcbauth_set_mode(authenticator: Authenticator, auth_type: AuthType) -> ErrorType;
    pub fn lcbauth_unref(authenticator: Authenticator);

    pub fn lcb_connect(instance: Instance) -> ErrorType;
    pub fn lcb_cntl_string(instance: Instance, key: *const c_char, value: *const c_char) -> ErrorType;
    pub fn lcb_create(instance: *mut Instance, options: *const CreateSt) -> ErrorType;
    pub fn lcb_destroy(instance: Instance);
    pub fn lcb_get3(instance: Instance, cookie: *const c_void, cmd: *const cmd::Get) -> ErrorType;
    pub fn lcb_get_bootstrap_status(instance: Instance) -> ErrorType;
    pub fn lcb_install_callback3(instance: Instance, cbtype: CallbackType, cb: ResponseCallback) -> ResponseCallback;
    pub fn lcb_remove3(instance: Instance, cookie: *const c_void, cmd: *const cmd::Remove) -> ErrorType;
    pub fn lcb_store3(instance: Instance, cookie: *const c_void, cmd: *const cmd::Store) -> ErrorType;
    pub fn lcb_strerror(instance: Instance, error: ErrorType) -> *const c_char;
    pub fn lcb_view_query(instance: Instance, cookie: *const c_void, cmd: *const cmd::ViewQuery) -> ErrorType;
    pub fn lcb_wait(instance: Instance) -> ErrorType;
}
