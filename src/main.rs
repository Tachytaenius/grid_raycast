// LEAVE NO TODOS! Check both files!

// TODO: Test for special cases, including rays on the grid lines
// TODO: Figure out raycasts starting on a tile edge
// TODO: Remove the print statements
// BUG: When you have a ray starting in the tile -1, -1, and it goes down by one pixel, the first non-initial result is -inf, and an assert fails

mod raycast;

use raycast::*;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 18;
const TILE_WIDTH: f32 = 30.5;
const TILE_HEIGHT: f32 = TILE_WIDTH; // TODO: Varied!
const GRID_OFFSET_X: f32 = 10.0;
const GRID_OFFSET_Y: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)

        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RayPoints {..default()})

        .add_startup_systems((
            create_grid,
            create_camera,
            create_ray
        ))

        .add_systems((
            update_ray,
            update_raycast
        ).chain())

        .run();
}

#[derive(Resource, Default)]
struct RayPoints { // More like a line segment with direction information
    start: Vec2,
    end: Vec2
}

#[derive(Component)]
struct RayDisplay;

#[derive(Component)]
struct IntersectionPoint;

fn create_ray(
    mut commands: Commands
) {
    commands.spawn((
        ShapeBundle {..default()},
        Fill::color(Color::YELLOW),
        Stroke::new(Color::YELLOW, 1.0),
        RayDisplay
    ));
}

fn create_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(
                window.width() / 2.0,
                window.height() / 2.0,
                999.9
            ),
            ..default()
        },
        ..default()
    });
}

fn create_grid(
    mut commands: Commands
) {
    let offset_vector = Vec2::new(GRID_OFFSET_X, GRID_OFFSET_Y);
    let scale_vector = Vec2::new(TILE_WIDTH, TILE_HEIGHT);

    for x in 0..=GRID_WIDTH {
        let x = x as f32;
        let line = shapes::Line(
            Vec2::new(x, 0.0) * scale_vector + offset_vector,
            Vec2::new(x, GRID_HEIGHT as f32) * scale_vector + offset_vector
        );
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&line),
                ..default()
            },
            Fill::color(Color::GRAY),
            Stroke::new(Color::GRAY, 1.0),
        ));
    }

    for y in 0..=GRID_HEIGHT {
        let y = y as f32;
        let line = shapes::Line(
            Vec2::new(0.0, y) * scale_vector + offset_vector,
            Vec2::new(GRID_WIDTH as f32, y) * scale_vector + offset_vector
        );
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&line),
                ..default()
            },
            Fill::color(Color::GRAY),
            Stroke::new(Color::GRAY, 1.0),
        ));
    }
}

fn update_ray(
    mut commands: Commands,
    mut ray: ResMut<RayPoints>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut ray_query: Query<Entity, With<RayDisplay>>
) {
    let window = window_query.get_single().unwrap();
    if let Some(mouse_position) = window.cursor_position() {
        if mouse_buttons.pressed(MouseButton::Left) {
            ray.start = mouse_position;
        }

        ray.end = mouse_position;
        if mouse_buttons.pressed(MouseButton::Right) {
            // ray.end = ray.start;

            // TEMP:
            ray.start = Vec2::new(40.0, 45.0)
        }
    }

    // Decided to compress this into a one-liner just for fun
    commands.entity(ray_query.get_single_mut().unwrap()).insert(GeometryBuilder::build_as(&shapes::Line(ray.start, ray.end)));
}

fn update_raycast(
    mut commands: Commands,
    ray: Res<RayPoints>,
    old_circle_query: Query<Entity, With<IntersectionPoint>>
) {
    for entity in old_circle_query.iter() {
        commands.entity(entity).despawn();
    }

    let raycast = line_tilemap_intersections_iterator_struct(ray.start, ray.end, TILE_WIDTH, Vec2::new(GRID_OFFSET_X, GRID_OFFSET_Y));
    for raycast_result in raycast {
        // TODO: Mark tiles
        println!("({}, {}), {}", raycast_result.tile_x, raycast_result.tile_y, raycast_result.intersection_t);
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    center: ray.start.lerp(ray.end, raycast_result.intersection_t),
                    radius: 2.0
                }),
                ..default()
            },
            Fill::color(Color::RED),
            Stroke::new(Color::RED, 1.0),
            IntersectionPoint
        ));
    }
    println!("");
}
