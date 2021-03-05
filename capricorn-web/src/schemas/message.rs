#[derive(juniper::GraphQLEnum)]
enum MessageType {
    Xml,
    Json,
    Url,
}
