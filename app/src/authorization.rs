use crate::query_dsl;
use serde::Deserialize;

#[derive(cynic::QueryFragment, Deserialize)]
#[cynic(
    schema_path = "schema.graphql",
    query_module = "query_dsl",
    graphql_type = "Authorities"
)]
#[serde(rename_all = "camelCase")]
pub struct Authorities {
    pub valid_token: bool,
}
