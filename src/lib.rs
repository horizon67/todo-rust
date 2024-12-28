use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub mod handlers;
pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
