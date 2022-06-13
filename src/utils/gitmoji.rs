use std::collections::HashMap;

pub fn replace(input: String) -> String {
    let mut result: String = String::from(input);

    gitmoji_map().iter().for_each(|(key, value)| {
        result = result.replace(key, value);
    });

    return String::from(result);
}

fn gitmoji_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from(":art:"), String::from("🎨"));
    map.insert(String::from(":zap:"), String::from("⚡️"));
    map.insert(String::from(":fire:"), String::from("🔥"));
    map.insert(String::from(":bug:"), String::from("🐛"));
    map.insert(String::from(":ambulance:"), String::from("🚑️"));
    map.insert(String::from(":sparkles:"), String::from("✨"));
    map.insert(String::from(":memo:"), String::from("📝"));
    map.insert(String::from(":rocket:"), String::from("🚀"));
    map.insert(String::from(":lipstick:"), String::from("💄"));
    map.insert(String::from(":tada:"), String::from("🎉"));
    map.insert(String::from(":white_check_mark:"), String::from("✅"));
    map.insert(String::from(":lock:"), String::from("🔒️"));
    map.insert(String::from(":closed_lock_with_key:"), String::from("🔐"));
    map.insert(String::from(":bookmark:"), String::from("🔖"));
    map.insert(String::from(":rotating_light:"), String::from("🚨"));
    map.insert(String::from(":construction:"), String::from("🚧"));
    map.insert(String::from(":green_heart:"), String::from("💚"));
    map.insert(String::from(":arrow_down:"), String::from("⬇️"));
    map.insert(String::from(":arrow_up:"), String::from("⬆️"));
    map.insert(String::from(":pushpin:"), String::from("📌"));
    map.insert(String::from(":construction_worker:"), String::from("👷"));
    map.insert(String::from(":chart_with_upwards_trend:"), String::from("📈"));
    map.insert(String::from(":recycle:"), String::from("♻️"));
    map.insert(String::from(":heavy_plus_sign:"), String::from("➕"));
    map.insert(String::from(":heavy_minus_sign:"), String::from("➖"));
    map.insert(String::from(":wrench:"), String::from("🔧"));
    map.insert(String::from(":hammer:"), String::from("🔨"));
    map.insert(String::from(":globe_with_meridians:"), String::from("🌐"));
    map.insert(String::from(":pencil2:"), String::from("✏️"));
    map.insert(String::from(":poop:"), String::from("💩"));
    map.insert(String::from(":rewind:"), String::from("⏪️"));
    map.insert(String::from(":twisted_rightwards_arrows:"), String::from("🔀"));
    map.insert(String::from(":package:"), String::from("📦️"));
    map.insert(String::from(":alien:"), String::from("👽️"));
    map.insert(String::from(":truck:"), String::from("🚚"));
    map.insert(String::from(":page_facing_up:"), String::from("📄"));
    map.insert(String::from(":boom:"), String::from("💥"));
    map.insert(String::from(":bento:"), String::from("🍱"));
    map.insert(String::from(":wheelchair:"), String::from("♿️"));
    map.insert(String::from(":bulb:"), String::from("💡"));
    map.insert(String::from(":beers:"), String::from("🍻"));
    map.insert(String::from(":speech_balloon:"), String::from("💬"));
    map.insert(String::from(":card_file_box:"), String::from("🗃️"));
    map.insert(String::from(":loud_sound:"), String::from("🔊"));
    map.insert(String::from(":mute:"), String::from("🔇"));
    map.insert(String::from(":busts_in_silhouette:"), String::from("👥"));
    map.insert(String::from(":children_crossing:"), String::from("🚸"));
    map.insert(String::from(":building_construction:"), String::from("🏗️"));
    map.insert(String::from(":iphone:"), String::from("📱"));
    map.insert(String::from(":clown_face:"), String::from("🤡"));
    map.insert(String::from(":egg:"), String::from("🥚"));
    map.insert(String::from(":see_no_evil:"), String::from("🙈"));
    map.insert(String::from(":camera_flash:"), String::from("📸"));
    map.insert(String::from(":alembic:"), String::from("⚗️"));
    map.insert(String::from(":mag:"), String::from("🔍️"));
    map.insert(String::from(":label:"), String::from("🏷️"));
    map.insert(String::from(":seedling:"), String::from("🌱"));
    map.insert(String::from(":triangular_flag_on_post:"), String::from("🚩"));
    map.insert(String::from(":goal_net:"), String::from("🥅"));
    map.insert(String::from(":dizzy:"), String::from("💫"));
    map.insert(String::from(":wastebasket:"), String::from("🗑️"));
    map.insert(String::from(":passport_control:"), String::from("🛂"));
    map.insert(String::from(":adhesive_bandage:"), String::from("🩹"));
    map.insert(String::from(":monocle_face:"), String::from("🧐"));
    map.insert(String::from(":coffin:"), String::from("⚰️"));
    map.insert(String::from(":test_tube:"), String::from("🧪"));
    map.insert(String::from(":necktie:"), String::from("👔"));
    map.insert(String::from(":stethoscope:"), String::from("🩺"));
    map.insert(String::from(":bricks:"), String::from("🧱"));
    map.insert(String::from(":technologist:"), String::from("🧑‍💻"));
    map.insert(String::from(":money_with_wings:"), String::from("💸"));

    map
}