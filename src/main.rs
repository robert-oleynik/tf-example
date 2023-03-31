use std::collections::HashSet;

use docker::resource::docker_container::*;
use docker::resource::docker_image::*;
use docker::Docker;
use tf_bindgen::Stack;

fn main() {
    let stack = Stack::new("postgres");

    Docker::create(&stack).build();

    let image = DockerImage::create(&stack, "postgres-image")
        .name("postgres:latest")
        .build();

    let mut env = HashSet::new();
    env.insert("POSTGRES_PASSWORD=example".to_string());

    DockerContainer::create(&stack, "postgres-container")
        .name("postgres")
        .image(&image.image_id)
        .env(["POSTGRES_PASSWORD=example"])
        .build();
}
