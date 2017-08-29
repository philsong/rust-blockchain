use amqp::Session;
use amqp::Channel;

pub fn init_rabbitmq() -> Channel {
    let session = Session::open_url("amqp:://localhost/").unwrap();
    let channel = session.open_channel(1).unwrap();

    channel
}