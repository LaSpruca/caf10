use actix::Message;

#[derive(Debug, Message)]
#[rtype(result = "")]
pub struct PlayerJoin(pub String);
