// penrose configuration
//
// Based on the example configuration "simple_config_with_hooks".
#[macro_use]
extern crate penrose;

use penrose::{
    contrib::hooks::LayoutSymbolAsRootName,
    core::{
        bindings::KeyBindings,
        config::Config,
        helpers::index_selectors,
        layout::{side_stack, Layout, LayoutConf},
        manager::WindowManager,
        ring::Selector,
        xconnection::XConn,
    },
    draw::{dwm_bar, Color, TextStyle},
    logging_error_handler,
    xcb::{XcbConnection, XcbDraw, XcbHooks},
    Backward, Forward, Less, More, PenroseError, Result,
};

use simplelog::{LevelFilter, SimpleLogger};
use std::collections::HashMap;
use std::convert::TryFrom;

// Default programs
const TERMINAL: &str = "alacritty";
const LAUNCHER: &str = "dmenu_run";
const BROWSER: &str = "firefox";
const PASSWORDS: &str = "passmenu";
const LOCKER: &str = "xscreensaver-command -lock";

// Style
const NORMAL: &str = "#9EEEEE";
const FOCUS: &str = "#55AAAA";
const GREY: &str = "#4D4D4D";
const NMAIN: u32 = 1;
const RATIO: f32 = 0.55;
const HEIGHT: usize = 18;
const FONT: &str = "xos4 Terminus";

fn create_config() -> Result<Config> {
    let mut config_builder = Config::default().builder();
    config_builder
        .workspaces(vec!["1", "2", "3", "4", "5", "6"])
        .floating_classes(vec!["dmenu"])
        .focused_border(FOCUS)?
        .unfocused_border(NORMAL)?
        .border_px(4)
        .gap_px(8)
        .show_bar(true);

    config_builder.layouts(vec![
        Layout::new("[T]", LayoutConf::default(), side_stack, NMAIN, RATIO),
        Layout::floating("[F]"),
    ]);

    config_builder.build().map_err(PenroseError::Raw)
}

fn create_hooks(config: &Config) -> Result<XcbHooks> {
    let hooks: XcbHooks = vec![
        LayoutSymbolAsRootName::new(),
        Box::new(dwm_bar(
            XcbDraw::new()?,
            HEIGHT,
            &TextStyle {
                font: FONT.to_string(),
                point_size: 12,
                fg: Color::try_from(FOCUS)?,
                bg: Some(Color::try_from(GREY)?),
                padding: (4.0, 4.0),
            },
            Color::try_from(GREY)?,
            Color::try_from(GREY)?,
            config.workspaces().clone(),
        )?),
    ];

    Ok(hooks)
}

fn create_bindings<X: XConn>() -> KeyBindings<X> {
    gen_keybindings! {
        // Program launch
        "M-semicolon" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);
        "M-f" => run_external!(BROWSER);
        "M-p" => run_external!(PASSWORDS);
        "M-l" => run_external!(LOCKER);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-c" => run_internal!(kill_client);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-bracketright" => run_internal!(cycle_screen, Forward);
        "M-bracketleft" => run_internal!(cycle_screen, Backward);
        "M-S-bracketright" => run_internal!(drag_workspace, Forward);
        "M-S-bracketleft" => run_internal!(drag_workspace, Backward);

        // Layout management
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);

        "M-A-s" => run_internal!(detect_screens);
        "M-S-q" => run_internal!(exit);

        map: { "1", "2", "3", "4", "5", "6" } to index_selectors(6) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    }
}

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Info, simplelog::Config::default())
        .expect("failed to init logging");

    let config = create_config().expect("Failed to build configuration!");
    let hooks = create_hooks(&config)?;
    let key_bindings = create_bindings();

    let conn = XcbConnection::new()?;
    let mut wm = WindowManager::new(config, conn, hooks, logging_error_handler());

    wm.init()?;
    wm.grab_keys_and_run(key_bindings, HashMap::new())?;

    Ok(())
}
