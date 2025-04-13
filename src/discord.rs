use crate::utils;

use discord_rich_presence::{
    activity::{Activity, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use std::sync::mpsc::Receiver;
use std::{
    thread::{sleep, spawn},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub fn discord_rpc(rx: Receiver<String>) {
    spawn(move || {
        let client_id = "1360671087637172315";

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let activity = Activity::new()
            .state("Voxiom.io Player abc")
            .details("Searching Stats")
            .buttons(vec![
                Button::new("Website", "https://tricko.pro"),
                Button::new("Community Server", "https://discord.gg/yPjrUrvSzv"),
            ])
            .timestamps(Timestamps::new().start(timestamp));

        let mut client: DiscordIpcClient = DiscordIpcClient::new(client_id).unwrap();
        client.connect().unwrap();
        client.set_activity(activity.clone()).unwrap();

        let mut detail: String = "Searching Stats".to_string();
        let is_game = |p: &str| matches!(p, "cryzen" | "kirka" | "vectaria" | "voxiom");

        loop {
            if let Some(new_url) = rx.try_iter().last() {
                if let Some(value) = new_url.splitn(2, "=").nth(1) {
                    let parts: Vec<&str> = value.split("/").skip(1).collect();
                    detail = match parts.as_slice() {
                        [p1] if is_game(p1) => format!("{} pate", utils::cap_str(p1)),
                        [p1, p2] if is_game(p1) => format!("{} {} page", utils::cap_str(p1), p2),
                        [p1, p2, p3] if is_game(p1) => {
                            format!("{} {}: {}", utils::cap_str(p1), p2, p3)
                        }
                        _ => "Searching Stats".to_string(),
                    };
                }
            }

            client
                .set_activity(activity.clone().state(&detail))
                .unwrap();

            sleep(Duration::from_secs(15));
        }
    });
}
