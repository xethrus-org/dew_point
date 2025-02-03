#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rusqlite::Connection;
use rocket::serde::Serialize;
use std::env;



#[derive(Serialize)]
struct Species {
    species_id: String,
    enclosure_size: String,
    humidity: i32,
    temp: i32,
}

#[get("/")]
fn index() -> rocket::response::Redirect {
    rocket::response::Redirect::to(uri!("/data"))
}

#[get("/data")]
fn get_data() -> Json<Vec<Species>> {
    let db_path = "db/sensor_data.db";
    let conn = Connection::open(db_path).expect("Failed to connect to database");
    let mut stmt = conn.prepare("SELECT species_id, enclosure_size, humidity, temperature FROM EnclosureDetails").unwrap();
    let species_iter = stmt.query_map([], |row| {
        Ok(Species {
            species_id: row.get(0)?,
            enclosure_size: row.get(1)?,
            humidity: row.get(2)?,
            temp: row.get(3)?,
        })
    }).unwrap();

    let species: Vec<Species> = species_iter.map(|s| s.unwrap()).collect();
    Json(species)
}

#[get("/<file..>")]
async fn files(file: std::path::PathBuf) -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(std::path::Path::new("html").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
     match env::current_dir() {
        Ok(path) => println!("Current directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }
    rocket::build().mount("/", routes![index, get_data, files])
}

