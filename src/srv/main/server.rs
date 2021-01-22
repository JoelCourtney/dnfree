use rocket::{Rocket, State};
use rocket_contrib::serve::StaticFiles;
use std::sync::RwLock;
use crate::character::{StoredCharacter,FinalCharacter};
use rocket::response::content;
use rocket_contrib::json::Json;
use serde::Deserialize;

struct SharedData {
    path: String,
    stored_char: RwLock<StoredCharacter>,
    final_char: RwLock<FinalCharacter>
}

pub(crate) fn ignite(path: String) -> Rocket {
    let mut stored_char = StoredCharacter::read(&*path);
    let final_char = stored_char.resolve().unwrap();
    let state = SharedData {
        path,
        stored_char: RwLock::new(stored_char),
        final_char: RwLock::new(final_char)
    };
    rocket::ignite()
        .manage(state)
        .mount("/", StaticFiles::from("src/www"))
        .mount("/", routes![get_character])
        .mount("/", routes![edit_character])
}

#[post("/")]
fn get_character(state: State<SharedData>) -> content::Json<String> {
    let final_char = state.inner().final_char.read().unwrap();
    content::Json(serde_json::to_string(&*final_char).expect("SERIALIZATION FAILED"))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "field", content = "value")]
enum EditRequest<'a> {
    Name(String),
    Health(u32),
    TempHealth(u32),
    Feature {
        container: &'a str,
        feature_index: usize,
        choice_index: usize,
        choice: &'a str
    }
}

#[post("/edit", format="json", data="<data>")]
fn edit_character(data: Json<EditRequest>, state: State<SharedData>) -> content::Json<String> {
    let mut final_char = state.inner().final_char.write().unwrap();
    let mut stored_char = state.inner().stored_char.write().unwrap();
    use EditRequest::*;
    match data.into_inner() {
        Name(s) => (*stored_char).name = s,
        Health(u) => (*stored_char).health = u,
        TempHealth(u) => (*stored_char).temp_health = u,
        Feature {
            container,
            feature_index,
            choice_index,
            choice
        } => {
            match match container {
                "race" => &mut (*final_char).race_traits,
                "class" => &mut (*final_char).class_features,
                "background" => &mut (*final_char).background_features,
                "feat" => &mut (*final_char).feat_features,
                _ => panic!(format!("no container found: {}", container))
            }.get_mut(feature_index)
                .expect(&format!("feature index out of bounds: {}", feature_index)).1 {
                Some(c) => unsafe {
                    (*c).choose(choice, choice_index);
                }
                None => panic!(format!("no choice here to choose from: {:?}", feature_index))
            }
        }
    }
    *final_char = stored_char.resolve().unwrap();
    std::mem::drop(final_char);
    stored_char.write(&*state.path);
    get_character(state)
}