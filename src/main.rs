#[macro_use]
extern crate rocket;
extern crate rdkafka;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rocket::State;
use rdkafka::util::Timeout;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::Message;



#[get("/")]
fn index() -> &'static str {
    "This is a simple Rust web app with two buttons."
}

#[get("/button1")]
async fn button1(producer: &State<FutureProducer>) -> &'static str {
    let topic = "topic-1";
    let value = "Button 1 Clicked !!!";
    let record = FutureRecord::to(topic).payload(value).key("button1");
    let timeout = Timeout::Never;
    
    producer.send(record, timeout).await.unwrap();

    "This is the first button."
}

#[get("/button2")]
async fn button2(consumer: &State<BaseConsumer>) -> &'static str {
    let timeout = std::time::Duration::from_secs(5);
    consumer
        .subscribe(&["topic-1"])
        .expect("Failed to subscribe to topic");
     

    loop {
        match consumer.poll(timeout) {
            Some(Ok(message)) => {
                if let Some(payload) = message.payload_view::<str>() {
                    println!("Received message: {}", payload.unwrap());
                }
            }
            Some(Err(e)) => {
                eprintln!("Error while receiving message: {:?}", e);
            }
            None => {
                println!("No Received message");
                break;
            }
        }
    }

    "This is the second button."
}


#[launch]
fn rocket() -> _ {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create::<FutureProducer>()
        .expect("Failed to create producer");
 
      
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "my-consumer-group")
        .create()
        .expect("Failed to create consumer");

    rocket::build()
        .manage(producer)
        .manage(consumer)
        .mount("/", routes![index, button1, button2])
}