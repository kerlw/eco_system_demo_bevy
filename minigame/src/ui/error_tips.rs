use bevy::{prelude::*, ui::ContentSize};
use bevy_tweening::{Animator, Delay, Tween, TweenCompleted, lens::*};
use std::time::Duration;

pub struct ErrorTipsPlugin;

const ERROR_TIPS_EVENT_ID: u64 = 0;

#[derive(Component)]
pub struct ErrorTipsLayerNode;

#[derive(Component)]
struct ErrorTipsUI;

// #[derive(Debug, Clone, Default)]
// struct ErrorTips {
//     pub content: String,    // 错误提示信息的内容
//     pub duration: Duration, // 错误提示信息显示的时长
// }

#[derive(Event)]
pub struct EventShowErrorTips(pub String);

impl Plugin for ErrorTipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_trigger_show_error_tips)
            .add_observer(on_complete_show_error_tips)
            .add_systems(Startup, setup_error_tips_layer);
    }
}

fn setup_error_tips_layer(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        ErrorTipsLayerNode,
        GlobalZIndex(99),
    ));
}

pub fn on_trigger_show_error_tips(
    trigger: Trigger<EventShowErrorTips>,
    mut commands: Commands,
    root_q: Query<Entity, With<ErrorTipsLayerNode>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(root) = root_q.single() {
        let font = asset_server.load("fonts/msyh.ttc");
        let tween = Tween::new(
            EaseFunction::CubicOut,
            Duration::from_secs_f32(0.6),
            UiPositionLens {
                start: UiRect::top(Val::Percent(8.0)),
                end: UiRect::top(Val::Percent(15.0)),
            },
        )
        .then(Delay::new(Duration::from_secs(1)).with_completed_event(ERROR_TIPS_EVENT_ID));

        commands.entity(root).with_children(|parent| {
            parent.spawn((
                Node {
                    max_width: Val::Percent(80.),
                    height: Val::Auto,
                    // position_type: PositionType::Absolute,
                    top: Val::Percent(8.0),
                    align_self: AlignSelf::Center,
                    // align_items: AlignItems::Center,
                    // justify_content: JustifyContent::Center,
                    padding: UiRect::horizontal(Val::Px(25.)),
                    ..Default::default()
                },
                ContentSize::default(),
                BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.3)),
                BorderRadius::all(Val::Px(5.0)),
                BorderColor(Color::srgba(0.6, 0.6, 0.6, 0.6)),
                // children![(
                Text::new(trigger.event().0.clone()),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(1.00, 0.19, 0.19)),
                TextLayout::new_with_justify(JustifyText::Center),
                // )],
                Animator::new(tween),
            ));
        });
    }
    // commands.
}

pub fn on_complete_show_error_tips(trigger: Trigger<TweenCompleted>, mut commands: Commands) {
    if trigger.event().user_data == ERROR_TIPS_EVENT_ID {
        commands.entity(trigger.target()).despawn();
    }
}

pub fn show_error_tips(commands: &mut Commands, tips: &str) {
    commands.trigger(EventShowErrorTips(tips.into()));
}
