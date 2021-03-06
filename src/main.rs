#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};

#[get("/")]
fn index() -> &'static str {
    "Hello, New Journey started!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

//A sleep route test
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds before execution", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, delay])
}
