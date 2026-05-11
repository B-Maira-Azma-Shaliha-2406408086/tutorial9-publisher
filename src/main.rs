use borsh_derive::{BorshDeserialize, BorshSerialize};
use crosstown_bus::CrosstownBus;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

fn main() {
    let mut p =
        CrosstownBus::new_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();

    _ = p.send(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406408086-Amir".to_owned(),
        },
    );

    _ = p.send(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406408086-Budi".to_owned(),
        },
    );

    _ = p.send(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406408086-Cica".to_owned(),
        },
    );

    _ = p.send(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406408086-Dira".to_owned(),
        },
    );

    _ = p.send(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406408086-Emir".to_owned(),
        },
    );
}