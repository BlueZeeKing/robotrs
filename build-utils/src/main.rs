use build_utils::{
    artifact::Artifact,
    zip::{extract_libs, get_zip},
};

#[tokio::main]
async fn main() {
    let artifact = Artifact::builder()
        .group_id("com.ctre.phoenix".to_string())
        .artifact_id("cci".to_string())
        .version("5.30.4".to_string())
        .target(build_utils::artifact::Target::RoboRio)
        .maven_url("https://maven.ctr-electronics.com/release/".to_string())
        .build()
        .unwrap();

    let mut archive = get_zip(&artifact.get_url()).await.unwrap();

    dbg!(extract_libs(&mut archive));
}
