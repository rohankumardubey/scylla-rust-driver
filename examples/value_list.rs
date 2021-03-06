use scylla::{Session, SessionBuilder};
use std::env;

#[tokio::main]
async fn main() {
    let uri = env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    println!("Connecting to {} ...", uri);

    let session: Session = SessionBuilder::new().known_node(uri).build().await.unwrap();

    session.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}", &[]).await.unwrap();

    session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.my_type (k int, my text, primary key (k))",
            &[],
        )
        .await
        .unwrap();

    #[derive(scylla::ValueList)]
    struct MyType {
        k: i32,
        my: Option<String>,
    }

    let to_insert = MyType {
        k: 17,
        my: Some("Some string".to_string()),
    };

    session
        .query("INSERT INTO ks.my_type (k, my) VALUES (?, ?)", to_insert)
        .await
        .unwrap();

    let q = session
        .query("SELECT * FROM ks.my_type", &[])
        .await
        .unwrap();

    println!("Q: {:?}", q.rows);
}
