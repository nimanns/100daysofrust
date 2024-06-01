#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    let result : i32 = 2+2;
    let result_as_string = result.to_string();

    result_as_string
}

// simple calculator example with 4 operations using route parameters
#[get("/calculator/<operation>/<num1>/<num2>")]
fn calculator(operation: String, num1: f32, num2: f32) -> String {
    let result : f32;
    match operation.as_str(){
        "add" => result = num1 + num2,
        "sub" => result = num1 - num2,
        "mul" => result = num1 * num2,
        "div" => result = num1 / num2,
        _ => return "Invalid operation".to_string()
    }
    result.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, calculator])
}