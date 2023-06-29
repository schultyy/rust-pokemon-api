use rocket::serde::json::Json;
use std::process::Command;

#[macro_use]
extern crate rocket;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Pokemon {
    #[serde(alias = "#")]
    number: i64,
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "Type 1")]
    type_1: String,
    #[serde(alias = "Type 2")]
    type_2: String,
    #[serde(alias = "Generation")]
    generation: i64,
}

fn fetch_pokemon_csv() -> Vec<Pokemon> {
    let pokemon_csv_output = Command::new("curl")
        .arg("-XGET")
        .arg("https://gist.githubusercontent.com/armgilles/194bcff35001e7eb53a2a8b441e8b2c6/raw/92200bc0a673d5ce2110aaad4544ed6c4010f687/pokemon.csv")
        .output()
        .expect("failed to execute process");
    let csv = String::from_utf8_lossy(&pokemon_csv_output.stdout).to_string();
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());

    let mut records = vec![];
    for result in rdr.deserialize() {
        let record: Pokemon = result.unwrap();
        records.push(record);
    }

    records
}

#[get("/all")]
fn all_pokemon() -> Json<Vec<Pokemon>> {
    let all_pokemon = fetch_pokemon_csv();
    Json(all_pokemon)
}

#[get("/?<generation>")]
fn specific_pokemon(generation: i64) -> Json<Vec<Pokemon>> {
    let all_pokemon = fetch_pokemon_csv();

    Json(
        all_pokemon
            .into_iter()
            .filter(|p| p.generation == generation)
            .collect::<Vec<_>>(),
    )
}

#[get("/health")]
fn health() -> String {
    return "OK".into()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health])
        .mount("/pokemon", routes![all_pokemon, specific_pokemon])
}
