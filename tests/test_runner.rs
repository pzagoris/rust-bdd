use api_tests::api::ApiWorld;
use cucumber::World;

mod step_definitions;

#[tokio::main]
async fn main() {
    ApiWorld::cucumber().run("tests/features").await;
}
