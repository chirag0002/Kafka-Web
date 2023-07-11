#[macro_use]
extern crate rocket;
extern crate rdkafka;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rocket::State;
use rdkafka::util::Timeout;

#[get("/")]
fn index() -> &'static str {
    "This is a simple Rust web app with two buttons."
}

#[get("/button1")]
async fn button1(producer: &State<FutureProducer>) -> &'static str {
    let topic = "sample";
    let value = "Hii!";
    let record = FutureRecord::to(topic).payload(value).key("button1");
    let timeout = Timeout::Never;
    
    producer.send(record, timeout).await.unwrap();

    "This is the first button."
}

#[get("/button2")]
async fn button2() -> &'static str {
    "This is the second button."
}


#[launch]
fn rocket() -> _ {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create::<FutureProducer>()
        .expect("Failed to create producer");

    
    rocket::build()
        .manage(producer)
        // .manage(consumer)
        .mount("/", routes![index, button1, button2])
}