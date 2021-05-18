// Copyright 2021 Andy King
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::menu::Menu;
use druid::menu::MenuItem;
use druid::widget::prelude::*;
use druid::LocalizedString;
use druid::SysMods;
use druid::WindowId;

use crate::common::commands;
use crate::model::app::AppState;

pub fn menu_bar(_: Option<WindowId>, _: &AppState, _: &Env) -> Menu<AppState> {
    #[cfg(target_os = "macos")]
    let base = druid::platform_menus::mac::menu_bar();

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    let base = base.entry(druid::platform_menus::win::file::default());

    base.entry(build_interpreter())
        .rebuild_on(|_old_data, _data, _env| false)
}

fn build_interpreter() -> Menu<AppState> {
    Menu::new(LocalizedString::new("Interpreter")).entry(
        MenuItem::new(LocalizedString::new("Go"))
            .enabled_if(|data: &AppState, _env| data.input.len() > 0)
            .hotkey(SysMods::Cmd, "g")
            .command(commands::INTERPRETER_GO),
    )
}
