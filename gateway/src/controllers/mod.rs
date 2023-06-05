use async_graphql::*;
use load_dotenv::load_dotenv;
mod schema;
mod querys;
mod mutations;
pub use querys::querys::QueryRoot;
pub use mutations::mutations::MutationRoot;

load_dotenv!();
#[derive(Debug)]
pub struct MyToken(pub String);
pub type UserSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;





