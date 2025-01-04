use services::llm_api;
use shared::event_system;

use crate::AsyaResponse;

pub async fn answer(user_message: String) {
    let answer = llm_api::send_request(user_message).await.unwrap();
    event_system::publish(AsyaResponse::Ok {
        message: answer.to_string(),
    })
    .await;
}
