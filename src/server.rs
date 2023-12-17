use ambient_api::{
    core::{
        app::components::name,
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        hierarchy::components::{children, parent},
        messages::Collision,
        physics::components::{cube_collider, dynamic, kinematic, visualize_collider},
        prefab::components::prefab_from_url,
        primitives::components::cube,
        transform::components::{local_to_parent, local_to_world, lookat_target, translation},
        transform::concepts::{Transformable, TransformableOptional},
    },
    prelude::*,
};

use packages::this::{assets, components::is_moving};

#[main]
pub fn main() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 10.),
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

    // Prefab Parent
    let parent_id = Transformable {
        optional: TransformableOptional {
            translation: Some(Vec3::Z * 5.0),
            ..default()
        },
        ..Transformable::suggested()
    }
    .make()
    .with(name(), "prefab-cube".to_string())
    .with(is_moving(), ())
    .spawn();

    // Prefab Cube
    let prefab_cube_id = Transformable {
        optional: TransformableOptional { ..default() },
        ..Transformable::suggested()
    }
    .make()
    .with(prefab_from_url(), assets::url("cube.glb"))
    .with(parent(), parent_id)
    .with(local_to_parent(), Default::default())
    .with(local_to_world(), Default::default())
    .with(visualize_collider(), ())
    .with(kinematic(), ())
    .with(name(), "prefab-cube".to_string())
    .spawn();

    entity::add_component(parent_id, children(), vec![]);
    entity::add_child(parent_id, prefab_cube_id);

    // // Runtime cube
    Transformable {
        optional: TransformableOptional {
            translation: Some(Vec3::Z * 5.0 + Vec3::X * 2.0),
            scale: Some(Vec3::ONE),
            ..default()
        },
        ..Transformable::suggested()
    }
    .make()
    .with(name(), "runtime-cube".to_string())
    .with(is_moving(), ())
    .with(cube(), ())
    .with(dynamic(), true)
    .with(cube_collider(), Vec3::ONE)
    .with(visualize_collider(), ())
    .spawn();

    query(is_moving()).each_frame(move |cubes| {
        for (eid, _name_value) in cubes {
            entity::mutate_component(eid, translation(), |v: &mut Vec3| {
                *v += Vec3::Z * -1.0 * delta_time();
            });
        }
    });
}
