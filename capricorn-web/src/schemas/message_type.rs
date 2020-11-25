#[derive(juniper::GraphQLEnum)]
enum MessageType {
    System,
    Message,
    Broadcast,
}

#[derive(juniper::GraphQLEnum)]
enum MessageContainer {
    Center,
    List,
    Detail,
}

#[derive(juniper::GraphQLEnum)]
enum MessageAging {
    RealTime,
    Timing, // 定时
}

/// The priority of the message
#[derive(juniper::GraphQLEnum)]
enum MessagePriority {
    Perferred,
    SecondPriority,
    Normal,
}

#[derive(juniper::GraphQLEnum)]
enum MessageRemindBy {
    RedPoint,
    ShowDetail,
    NotificationBar,
    Banner,
}

#[derive(juniper::GraphQLEnum)]
enum MessageTogetherWith {
    Sound,
    Font,
    Color,
}