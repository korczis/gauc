extern crate gauc;
extern crate env_logger;

use gauc::client::*;

fn main() {
    env_logger::init().unwrap();

    if let Ok(mut client) = Client::connect("couchbase://localhost/default", None) {
        let res = client.upsert_sync("hello_world_upsert_sync", "{{\"msg\": \"This is sync upsert!\"}}", 0, 0);
        println!("{:?}", res);

        let res = client.get_sync("hello_world_upsert_sync");
        println!("{:?}", res);
    }
}
