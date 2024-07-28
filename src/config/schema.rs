use async_graphql::*;
use sea_orm::DbConn;

use crate::files::graphql::mutations::FileMutations;

#[derive(Default)]
struct VersionQuery;

#[Object]
impl VersionQuery {
    async fn version(&self) -> String {
        String::from("0.0.1")
    }
}

#[derive(MergedObject, Default)]
pub struct Query(VersionQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(FileMutations);

type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema(conn: DbConn) -> AppSchema {
    AppSchema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(conn)
        .finish()
}
