use crate::{Client, ConversationInfo, ManagerError, Database, db_interactions::db_interactions_mongo::get_db,};
use bson::{doc, Bson};

pub fn create_node(data: &mut ConversationInfo) -> Result<(), ManagerError> {
    let time = Bson::UtcDatetime(chrono::Utc::now());
    let node = doc! {
        "client": bson::to_bson(&data.client)?,
        "interaction_id": &data.interaction_id,
        "conversation_id": &data.conversation_id,
        "flow_id": &data.context.flow,
        "step_id": &data.context.step,
        "next_flow": Bson::Null, //"Option<string>",
        "next_step": Bson::Null, //"Option<string>",
        "created_at": time
    };

    let db = get_db(&data.db)?;
    let path = db.collection("path");

    path.insert_one(node, None)?;

    Ok(())
}
