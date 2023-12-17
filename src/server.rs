use ambient_api::{
    core::{
        app::components::name,
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        messages::Collision,
        physics::components::{cube_collider, dynamic, kinematic, visualize_collider},
        prefab::components::prefab_from_url,
        primitives::components::cube,
        transform::components::{lookat_target, translation},
        transform::concepts::{Transformable, TransformableOptional},
    },
    prelude::*,
};

use packages::this::assets;

#[main]
pub fn main() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 5.),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    .with(lookat_target(), vec3(0., 0., 0.))
    .spawn();

    Collision::subscribe(move |msg| {
        let ids_str = msg
            .ids
            .iter()
            .map(|id| {
                let mut name_string = "unkown".to_string();
                if entity::has_component(*id, name()) {
                    name_string = entity::get_component(*id, name()).unwrap();
                }
                name_string
            })
            .collect::<Vec<_>>()
            .join(", ");

        println!("collision detected! {:?}", ids_str);
    });

    // Ground
    Transformable {
        optional: TransformableOptional { ..default() },
        ..Transformable::suggested()
    }
    .make()
    .with(name(), "ground".to_string())
    .with(prefab_from_url(), assets::url("ground.glb"))
    .spawn();

    // Prefab Cube
    Transformable {
        optional: TransformableOptional {
            translation: Some(Vec3::Z * 5.0 + Vec3::X * -2.0),
            ..default()
        },
        ..Transformable::suggested()
    }
    .make()
    .with(name(), "prefab-cube".to_string())
    .with(prefab_from_url(), assets::url("cube.glb"))
    .with(dynamic(), true)
    .with(kinematic(), ())
    .with(visualize_collider(), ())
    .spawn();

    // Runtime cube
    Transformable {
        optional: TransformableOptional {
            translation: Some(Vec3::Z * 5.0 + Vec3::X * 2.0),
            scale: Some(Vec3::ONE * 2.0),
            ..default()
        },
        ..Transformable::suggested()
    }
    .make()
    .with(name(), "runtime-cube".to_string())
    .with(cube(), ())
    .with(dynamic(), true)
    .with(cube_collider(), Vec3::ONE * 1.0)
    .with(visualize_collider(), ())
    .spawn();

    query(name()).each_frame(move |cubes| {
        for (eid, _name_value) in cubes {
            entity::mutate_component(eid, translation(), |v: &mut Vec3| {
                *v += Vec3::Z * -1.0 * delta_time();
            });
        }
    });
}
