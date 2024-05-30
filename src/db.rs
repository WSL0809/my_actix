use crate::schema::User;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
pub type UserDb = Arc<Mutex<HashMap<u32, User>>>;
