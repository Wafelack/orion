/*
 *  Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
 *
 *  ------------------------------------------------------
 *
 *     This file is part of Orion.
 *
 *  Orion is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Orion is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Orion.  If not, see <https://www.gnu.org/licenses/>.
 */
#[derive(Debug)]
pub struct OrionError(pub Option<String>, pub Option<usize>, pub String);

pub type Result<T> = std::result::Result<T, OrionError>;

#[macro_export]
macro_rules! error {
    ($($arg:tt)* $(, $file:expr, $line:expr)?) => {
        {
            let mut file = None;
            let mut line = None;
            $ (
                file = Some($file.to_string());
                line = Some($line)
              )?
                Err(OrionError(file, line, format_args!($($arg)*).to_string()))

        }
    }
}
