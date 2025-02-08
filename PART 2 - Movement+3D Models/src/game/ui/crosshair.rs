use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_crosshair(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();
    let crosshair_size = 2.0;

    // Spawn a root UI node that covers the entire window.
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Other layout properties (like alignment) can be set here if needed.
                ..default()
            },
            // Use a transparent background for the root node.
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Spawn the crosshair as a child UI node.
            parent.spawn((
                // Instead of UiImage::solid_color, use ImageNode’s constructor for a solid color.
                // (This helper function may be provided as `ImageNode::from_color` or similar.)
                // UiImage::solid_color(Color::srgb(0., 1., 0.)),
                Node {
                    // Set the size of the crosshair.
                    width: Val::Px(crosshair_size),
                    height: Val::Px(crosshair_size),
                    // Specify that the node is positioned absolutely within its parent.
                    position_type: PositionType::Absolute,
                    // Center it by offsetting from the parent's top‑left corner.
                    left: Val::Px(window.width() / 2.0 - crosshair_size / 2.0),
                    top: Val::Px(window.height() / 2.0 - crosshair_size / 2.0),
                    ..default()
                },
            ));
        });
}
