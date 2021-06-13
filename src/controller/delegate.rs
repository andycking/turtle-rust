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

use druid::DelegateCtx;
use druid::Env;
use druid::Handled;
use druid::Target;

use crate::common::commands;
use crate::model::app::AppState;

pub struct Delegate;

impl druid::AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        match cmd {
            _ if cmd.is(commands::INTERPRETER_GO) => {
                super::interpreter::go(ctx, cmd, data);
                Handled::Yes
            }

            _ if cmd.is(commands::INTERPRETER_SPEED) => {
                super::interpreter::speed(ctx, cmd, data);
                Handled::Yes
            }

            _ if cmd.is(commands::EXAMPLES) => {
                super::examples::show(ctx, cmd, data);
                Handled::Yes
            }

            _ => Handled::No,
        }
    }
}
