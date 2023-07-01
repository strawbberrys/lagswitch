#![windows_subsystem = "windows"]

use druid::widget::Button;
use druid::{AppDelegate, AppLauncher, Data, Env, Widget, WindowDesc};

mod lagswitch;

#[derive(Clone, Data, Default)]
struct AppState {
    enabled: bool,
}

struct Delegate;

fn main() {
    let window = WindowDesc::new(ui_builder())
        .title("Lagswitch")
        .with_min_size((200.0, 200.0))
        .window_size((200.0, 200.0));

    AppLauncher::with_window(window)
        .delegate(Delegate)
        .launch(AppState::default())
        .expect("Failed to launch");
}

fn ui_builder() -> impl Widget<AppState> {
    let button = Button::new(
        |data: &AppState, _env: &Env| {
            if data.enabled {
                "Enabled"
            } else {
                "Disabled"
            }
        },
    )
    .on_click(|_ctx, data: &mut AppState, _env| {
        data.enabled = !data.enabled;

        if data.enabled {
            lagswitch::enable();
        } else {
            lagswitch::disable();
        }
    });

    button
}

impl AppDelegate<AppState> for Delegate {
    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
        lagswitch::disable();
        data.enabled = false;
    }
}
