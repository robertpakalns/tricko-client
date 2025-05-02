use discord_rich_presence::{
    activity::{Activity, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::{
    sync::Mutex,
    thread::{sleep, spawn},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

static CURRENT_DETAILS: Mutex<String> = Mutex::new(String::new());

#[tauri::command]
pub fn drpc_init() {
    spawn(move || {
        let client_id = "1360671087637172315";

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let activity = Activity::new()
            .state("Searching Stats")
            .details("Searching Stats")
            .buttons(vec![
                Button::new("GitHub", "https://github.com/robertpakalns/tricko-client"),
                Button::new("Community Server", "https://discord.gg/yPjrUrvSzv"),
            ])
            .timestamps(Timestamps::new().start(timestamp));

        let mut client: DiscordIpcClient = DiscordIpcClient::new(client_id).unwrap();
        client.connect().unwrap();
        client.set_activity(activity.clone()).unwrap();

        loop {
            let detail = CURRENT_DETAILS.lock().unwrap().clone();

            client
                .set_activity(activity.clone().state(&detail))
                .unwrap();

            sleep(Duration::from_secs(15));
        }
    });
}

#[tauri::command]
pub fn drpc_set_detail(text: &str) {
    let mut current = CURRENT_DETAILS.lock().unwrap();
    *current = text.to_string();
}
