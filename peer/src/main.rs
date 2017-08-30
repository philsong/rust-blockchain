#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate amqp;
extern crate crypto;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate time;
extern crate uuid;

#[macro_use]
extern crate serde_derive;

mod block;
mod blockchain;
mod connections;
mod guards;

fn main() {
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    let channel = connections::init_rabbitmq();

    rocket::ignite()
        .manage(connections::init_postgres())
        .mount(
            "/api/block",
            routes![block::resources::new],
        )
        .mount(
            "/api/blockchain",
            routes![blockchain::resources::new, blockchain::resources::overview],
        )
}