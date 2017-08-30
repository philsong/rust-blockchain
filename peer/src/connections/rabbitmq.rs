use amqp::{Channel, Session};

pub fn init_rabbitmq() -> Channel {
    let mut session: Session = match Session::open_url("amqp://localhost//") {
        Ok(session) => session,
        Err(error) => panic!("Error during RabbitMQ connection. {}", error),
    };

    session.open_channel(1).unwrap()
}