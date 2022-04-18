use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
};

pub struct ClickingPlugin;

#[derive(Component, Debug)]
pub struct DoubleclickDuration(f64);

#[derive(Component, Debug)]
pub struct ClickDuration(f64);

#[derive(Component)]
pub struct Button(MouseButton);

#[derive(Component)]
pub struct ClickTime(f64);

#[derive(Component)]
pub struct DoubleclickTime(f64);

#[derive(Bundle)]
pub struct ClickDetectorBundle {
    button: Button,
    duration: ClickDuration,
    press_time: ClickTime,
}

#[derive(Bundle)]
pub struct DoubleClickDetectorBundle {
    #[bundle]
    cdb: ClickDetectorBundle,
    duration: DoubleclickDuration,
    click_time: DoubleclickTime,
}

pub struct ClickEvent {
    pub button: MouseButton,
}

pub struct DoubleclickEvent {
    pub button: MouseButton,
}

fn click_detector(
    mut query: Query<(&Button, &ClickDuration, &mut ClickTime)>,
    mut button_ev: EventReader<MouseButtonInput>,
    mut click_ev: EventWriter<ClickEvent>,
    time: Res<Time>,
) {
    for ev in button_ev.iter() {
        for (button, duration, mut press_time) in query.iter_mut() {
            if button.0 == ev.button {
                match ev.state {
                    ElementState::Pressed => {
                        press_time.0 = time.seconds_since_startup();
                    }
                    ElementState::Released => {
                        if (time.seconds_since_startup() - press_time.0) <= duration.0 {
                            click_ev.send(ClickEvent { button: button.0 });
                        }
                    }
                }
            }
        }
    }
}

fn double_click_detector(
    mut query: Query<(&Button, &DoubleclickDuration, &mut DoubleclickTime)>,
    mut click_ev: EventReader<ClickEvent>,
    mut double_ev: EventWriter<DoubleclickEvent>,
    time: Res<Time>,
) {
    for ev in click_ev.iter() {
        for (button, duration, mut click_time) in query.iter_mut() {
            if button.0 == ev.button {
                if (time.seconds_since_startup() - click_time.0) <= duration.0 {
                    double_ev.send(DoubleclickEvent { button: button.0 });
                    click_time.0 = 0.0;
                } else {
                    click_time.0 = time.seconds_since_startup();
                }
            }
        }
    }
}

fn plugin_init(mut commands: Commands) {
    commands.spawn_bundle(DoubleClickDetectorBundle {
        cdb: ClickDetectorBundle {
            button: Button(MouseButton::Left),
            duration: ClickDuration(0.1),
            press_time: ClickTime(0.0),
        },
        duration: DoubleclickDuration(0.3),
        click_time: DoubleclickTime(0.0),
    });
    commands.spawn_bundle(DoubleClickDetectorBundle {
        cdb: ClickDetectorBundle {
            button: Button(MouseButton::Middle),
            duration: ClickDuration(0.1),
            press_time: ClickTime(0.0),
        },
        duration: DoubleclickDuration(0.3),
        click_time: DoubleclickTime(0.0),
    });
    commands.spawn_bundle(DoubleClickDetectorBundle {
        cdb: ClickDetectorBundle {
            button: Button(MouseButton::Right),
            duration: ClickDuration(0.1),
            press_time: ClickTime(0.0),
        },
        duration: DoubleclickDuration(0.3),
        click_time: DoubleclickTime(0.0),
    });
}

impl Plugin for ClickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(plugin_init)
            .add_system(click_detector)
            .add_system(double_click_detector)
            .add_event::<ClickEvent>()
            .add_event::<DoubleclickEvent>();
    }
}
