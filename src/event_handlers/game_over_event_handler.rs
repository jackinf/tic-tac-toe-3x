use crate::events::GameOverEvent;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{
    AlignItems, AssetServer, Color, Commands, EventReader, JustifyContent, NodeBundle,
    PositionType, Res, Style, TextBundle, TextStyle, Val,
};

pub fn game_over_event_handler(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in game_over_event_reader.read() {
        let winner = event.winner();

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.with_a(0.5).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Game Over!",
                        TextStyle {
                            font: asset_server.load("fonts/AmericanCaptain.ttf"),
                            font_size: 200.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Relative,
                        ..Default::default()
                    }),
                );

                // show score
                parent.spawn(
                    TextBundle::from_section(
                        &format!("Winner: {}", winner),
                        TextStyle {
                            font: asset_server.load("fonts/AmericanCaptain.ttf"),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        position_type: PositionType::Relative,
                        top: Val::Px(200.0),
                        ..Default::default()
                    }),
                );
            });
    }
}
