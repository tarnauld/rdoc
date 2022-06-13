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

pub fn gitmoji_description() -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert(String::from("🎨"), String::from("Improve structure / format of the code."));
    map.insert(String::from("⚡️"), String::from("Improve performance."));
    map.insert(String::from("🔥"), String::from("Remove code or files."));
    map.insert(String::from("🐛"), String::from("Fix a bug."));
    map.insert(String::from("🚑️"), String::from("Critical hotfix."));
    map.insert(String::from("✨"), String::from("Introduce new features."));
    map.insert(String::from("📝"), String::from("Add or update documentation."));
    map.insert(String::from("🚀"), String::from("Deploy stuff."));
    map.insert(String::from("💄"), String::from("Add or update the UI and style files."));
    map.insert(String::from("🎉"), String::from("Begin a project."));
    map.insert(String::from("✅"), String::from("Add, update, or pass tests."));
    map.insert(String::from("🔒️"), String::from("Fix security issues."));
    map.insert(String::from("🔐"), String::from("Add or update secrets."));
    map.insert(String::from("🔖"), String::from("Release / Version tags."));
    map.insert(String::from("🚨"), String::from("Fix compiler / linter warnings."));
    map.insert(String::from("🚧"), String::from("Work in progress."));
    map.insert(String::from("💚"), String::from("Fix CI Build."));
    map.insert(String::from("⬇️"), String::from("Downgrade dependencies."));
    map.insert(String::from("⬆️"), String::from("Upgrade dependencies."));
    map.insert(String::from("📌"), String::from("Pin dependencies to specific versions."));
    map.insert(String::from("👷"), String::from("Add or update CI build system."));
    map.insert(String::from("📈"), String::from("Add or update analytics or track code."));
    map.insert(String::from("♻️"), String::from("Refactor code."));
    map.insert(String::from("➕"), String::from("Add a dependency."));
    map.insert(String::from("➖"), String::from("Remove a dependency."));
    map.insert(String::from("🔧"), String::from("Add or update configuration files."));
    map.insert(String::from("🔨"), String::from("Add or update development scripts."));
    map.insert(String::from("🌐"), String::from("Internationalization and localization."));
    map.insert(String::from("✏️"), String::from("Fix typos."));
    map.insert(String::from("💩"), String::from("Write bad code that needs to be improved."));
    map.insert(String::from("⏪️"), String::from("Revert changes."));
    map.insert(String::from("🔀"), String::from("Merge branches."));
    map.insert(String::from("📦️"), String::from("Add or update compiled files or packages."));
    map.insert(String::from("👽️"), String::from("Update code due to external API changes."));
    map.insert(String::from("🚚"), String::from("Move or rename resources (e.g.: files, paths, routes)."));
    map.insert(String::from("📄"), String::from("Add or update license."));
    map.insert(String::from("💥"), String::from("Introduce breaking changes."));
    map.insert(String::from("🍱"), String::from("Add or update assets."));
    map.insert(String::from("♿️"), String::from("Improve accessibility."));
    map.insert(String::from("💡"), String::from("Add or update comments in source code."));
    map.insert(String::from("🍻"), String::from("Write code drunkenly."));
    map.insert(String::from("💬"), String::from("Add or update text and literals."));
    map.insert(String::from("🗃️"), String::from("Perform database related changes."));
    map.insert(String::from("🔊"), String::from("Add or update logs."));
    map.insert(String::from("🔇"), String::from("Remove logs."));
    map.insert(String::from("👥"), String::from("Add or update contributor(s)."));
    map.insert(String::from("🚸"), String::from("Improve user experience / usability."));
    map.insert(String::from("🏗️"), String::from("Make architectural changes."));
    map.insert(String::from("📱"), String::from("Work on responsive design."));
    map.insert(String::from("🤡"), String::from("Mock things."));
    map.insert(String::from("🥚"), String::from("Add or update an easter egg."));
    map.insert(String::from("🙈"), String::from("Add or update a .gitignore file."));
    map.insert(String::from("📸"), String::from("Add or update snapshots."));
    map.insert(String::from("⚗️"), String::from("Perform experiments."));
    map.insert(String::from("🔍️"), String::from("Improve SEO."));
    map.insert(String::from("🏷️"), String::from("Add or update types."));
    map.insert(String::from("🌱"), String::from("Add or update seed files."));
    map.insert(String::from("🚩"), String::from("Add, update, or remove feature flags."));
    map.insert(String::from("🥅"), String::from("Catch errors."));
    map.insert(String::from("💫"), String::from("Add or update animations and transitions."));
    map.insert(String::from("🗑️"), String::from("Deprecate code that needs to be cleaned up."));
    map.insert(String::from("🛂"), String::from("Work on code related to authorization, roles and permissions."));
    map.insert(String::from("🩹"), String::from("Simple fix for a non-critical issue."));
    map.insert(String::from("🧐"), String::from("Data exploration/inspection."));
    map.insert(String::from("⚰️"), String::from("Remove dead code."));
    map.insert(String::from("🧪"), String::from("Add a failing test."));
    map.insert(String::from("👔"), String::from("Add or update business logic"));
    map.insert(String::from("🩺"), String::from("Add or update healthcheck."));
    map.insert(String::from("🧱"), String::from("Infrastructure related changes."));
    map.insert(String::from("🧑‍💻"), String::from("Improve developer experience"));
    map.insert(String::from("💸"), String::from("Add sponsorships or money related infrastructure."));

    map
}