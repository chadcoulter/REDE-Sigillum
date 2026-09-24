use cucumber::World as _;

#[derive(Debug, Default, cucumber::World)]
struct World;

#[tokio::main]
async fn main() {
    World::run("../../features").await;
}
