use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use self::user::QueryRoot;

pub mod user;

pub type IndexSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
