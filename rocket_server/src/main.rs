#[macro_use]
extern crate rocket;
use todo;

#[get("/")]
fn index() -> String {
    format!("Hello {name}", name = "Rocket")
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index]).mount("/todo", routes![todo::get_todos, todo::add_todo, todo::get_todo, todo::update_todo, todo::delete_todo])
}
