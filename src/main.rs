use mongodb::bson::{doc, Bson, Document};

#[tokio::main]
async fn main() {
    let now: mongodb::bson::DateTime = chrono::Utc::now().into();
    let db: mongodb::Database = mongodb::Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap()
        .database("test");
    db.collection::<Document>("mongodb_bson_datetime_bug")
        .insert_one(
            doc! {
                "user_id": 1,
                "updated_at": Bson::DateTime(now)
            },
            None,
        )
        .await
        .unwrap();
    db.collection::<Document>("mongodb_bson_datetime_bug")
        .update_one(
            doc! {"user_id": 1},
            doc! {
                "$set": {
                    "updated_at": Bson::DateTime(now)
                }
            },
            None,
        )
        .await
        .unwrap(); // panic: BsonSerialization(SubMillisecondPrecisionDateTime(DateTime(2021-05-20T06:48:38.101442962Z)
}
