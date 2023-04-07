//! This module defines handlers that can be called from JavaScript.

use std::sync::Arc;

use anyhow::anyhow;
use tauri::{
    command, generate_handler, AppHandle, InvokeHandler, Manager, State, Window, WindowBuilder,
    WindowUrl, Wry,
};
use tokio::sync::Notify;

use game_controller::action::VAction;

use crate::launch::{LaunchData, LaunchSettings};
use crate::runtime::{start_runtime, RuntimeState};

/// This struct is used as state so that the [launch] function can communicate to
/// [sync_with_backend] that the full [RuntimeState] is managed now.
struct SyncState(Arc<Notify>);

/// This function is called by the launcher to obtain its data. The data is read from a state
/// variable that is created by [crate::launch::make_launch_data] and put there by [crate::main].
#[command]
fn get_launch_data(launch_data: State<LaunchData>) -> LaunchData {
    launch_data.inner().clone()
}

/// This function is called when the user finishes the launcher window. It closes the launcher
/// window, creates a game state, network services, and the main window, and spawns tasks to handle
/// events.
#[command]
async fn launch(settings: LaunchSettings, window: Window, app: AppHandle) {
    assert_eq!(window.label(), "launcher");

    // The notify object must be managed before the window is created.
    let runtime_notify = Arc::new(Notify::new());
    app.manage(SyncState(runtime_notify.clone()));

    // The window is created here so we can have a reference to it in send_ui_state without looking
    // up the window by name each time (and other stuff would be complicated as well).
    let main_window = WindowBuilder::new(&app, "main", WindowUrl::App("main.html".into()))
        .center()
        .min_inner_size(1024.0, 768.0)
        .title("GameController")
        .fullscreen(settings.window.fullscreen)
        .build()
        .unwrap();

    // At least on Linux, the launcher must be closed after the main window was created, because
    // otherwise the application wants to exit.
    window.close().unwrap();

    let send_ui_state = move |ui_state| {
        if let Err(error) = main_window.emit("state", ui_state) {
            Err(anyhow!(error))
        } else {
            Ok(())
        }
    };

    let launch_data = app.state::<LaunchData>();
    app.manage(
        start_runtime(
            // TODO: This will probably not work in production.
            &app.path_resolver()
                .resource_dir()
                .unwrap()
                .join("..")
                .join("..")
                .join("config"),
            &app.path_resolver()
                .resource_dir()
                .unwrap()
                .join("..")
                .join("..")
                .join("logs"),
            &settings,
            &launch_data.teams,
            &launch_data.network_interfaces,
            Box::new(send_ui_state),
        )
        .await
        .unwrap(),
    );

    // Now that the RuntimeState is managed, we can tell the UI that it can proceed.
    runtime_notify.notify_one();
}

/// This function should be called once by the UI after it listens to UI events, but before it
/// calls [apply_action]. The result is needed as a tauri workaround.
#[command]
async fn sync_with_backend(app: AppHandle, state: State<'_, SyncState>) -> Result<(), ()> {
    // Wait until manage has been called.
    state.0.notified().await;
    // Now we can obtain a handle to the RuntimeState to notify the runtime thread that it can
    // start sending UI events.
    app.state::<RuntimeState>().ui_notify.notify_one();
    Ok(())
}

/// This function enqueues an action to be applied to the game.
#[command]
fn apply_action(action: VAction, state: State<RuntimeState>) {
    let _ = state.action_sender.send(action);
}

/// This function returns a handler that can be passed to [tauri::Builder::invoke_handler].
/// It must be boxed because otherwise its size is unknown at compile time.
pub fn get_invoke_handler() -> Box<InvokeHandler<Wry>> {
    Box::new(generate_handler![
        apply_action,
        get_launch_data,
        launch,
        sync_with_backend,
    ])
}