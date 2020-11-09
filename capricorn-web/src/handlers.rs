use juniper::{EmptyMutation, RootNode};

use crate::schemas::root::QueryRoot;

type Schema = RootNode<'static, QueryRoot, EmptyMutation<Database>, Subscription>;
