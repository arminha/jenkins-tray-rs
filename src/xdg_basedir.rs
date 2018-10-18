/*
Copyright (C) 2017  Armin Häberling

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use std::env;
use std::path::PathBuf;

pub fn config_home() -> PathBuf {
    if let Some(config_home) = env::var_os("XDG_CONFIG_HOME") {
        PathBuf::from(config_home)
    } else {
        // fallback: $HOME/.config
        let home = env::var_os("HOME").expect("HOME environment variable");
        let mut path = PathBuf::from(home);
        path.push(".config");
        path
    }
}
