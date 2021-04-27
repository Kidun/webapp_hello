#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[get("/world")]
fn world() -> String {
    format!("Hello, world! v.{}", VERSION)
}

#[get("/мир")]
fn mir() -> String {
    format!("Привет, мир! v.{}", VERSION)
}

#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "ру")]
    Russian
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

// Note: without the `..` in `opt..`, we'd need to pass `opt.emoji`, `opt.name`.
#[get("/?<lang>&<opt..>")]
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("👋 ");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![world, mir])
        .mount("/wave", routes![wave])
}
