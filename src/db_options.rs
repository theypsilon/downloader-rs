// Copyright (c) 2021-2025 José Manuel Barroso Galindo <theypsilon@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// You can download the latest version of this tool from:
// https://github.com/theypsilon/downloader-rs

//! A struct to hold database options and functions to create and manipulate those options.

#![allow(non_snake_case)]
#![allow(dead_code)] // TODO - remove this. It is currently necessary because nothing calls this code.

#[derive(Debug)]
/// This struct represents a database options configuration.
pub struct DbOptions {}

impl DbOptions {}

/// An error struct used to report errors when creating the DbOptions struct.
pub struct DbOptionsValidationError {}

impl DbOptionsValidationError {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_construct_db_options___with_correct_props___returns_options() {
        // self.assertIsNotNone(db_options())
    }

    #[test]
    fn test_construct_db_options___with_empty_props___returns_empty_options() {
        // self.assertEqual({}, DbOptions({}).unwrap_props())
    }

    #[test]
    fn test_construct_db_options___with_not_recognized_option___returns_error() {
        // self.assertRaises(DbOptionsValidationException, lambda: DbOptions({'wrong': 'option'}))
    }

    fn test_construct_db_options___with_option_base_path___raises_nothing_by_now() {
        // self.assertIsNotNone(DbOptions({'base_path': 'something'}))
    }
}
