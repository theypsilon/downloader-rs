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

use std::collections::HashMap;

#[derive(Debug)]
/// This struct represents a database options configuration.
pub struct DbOptions {
    /// A set of strings representing the currently enabled options.
    pub present: Vec<String>,
    /// An optional string representing a filter to apply to the data.
    pub filter: Option<String>,
    /// A HashMap where keys are string identifiers and values are string identifiers representing the properties of
    /// each database object.
    pub _props: HashMap<String, String>,
}

impl DbOptions {
    /// Creates a new `DbOptions` instance from a map of properties, validating them for correctness.
    ///
    /// # Arguments:
    ///
    /// * `props` - properties to set
    ///
    /// # Returns:
    ///
    /// * On success, a newly constructed DbOptions struct.
    /// * On failure, an error struct.
    ///
    pub fn new(props: HashMap<String, String>) -> Result<Self, DbOptionsValidationError> {
        let mut present = Vec::new();
        let filter: Option<String>;

        if props.contains_key("filter") {
            present.push("filter".to_string());
            match props.get("filter") {
                Some(value) => filter = Some(value.clone()),
                // This should never happen because of the contains, above.
                None => filter = None,
            }
        } else {
            filter = None;
        }

        if present.len() != props.len() {
            let mut validation_error = DbOptionsValidationError::new();
            props.keys().for_each(|o| {
                if !present.contains(o) {
                    validation_error.insert(o.to_string());
                }
            });

            return Err(validation_error);
        }

        Ok(DbOptions {
            present: present,
            filter: filter,
            _props: props,
        })
    }

    /// Report if any filter options are set.
    ///
    /// # Returns:
    ///
    /// - true if any filter options are set
    /// - false if no filter options are set
    ///
    pub fn any(self) -> bool {
        self.filter.is_some()
    }

    /// Unwrap the struct's properties and return them.
    ///
    /// # Returns:
    ///
    /// * None if props is empty
    /// * A Some containing the props if props is not empty.
    ///
    pub fn unwrap_props(self) -> Option<HashMap<String, String>> {
        if self._props.is_empty() {
            return None;
        } else {
            Some(self._props)
        }
    }
}

/// An error struct used to report errors when creating the DbOptions struct.
pub struct DbOptionsValidationError {
    /// A list of field names which failed validation.
    fields: Vec<String>,
}

impl DbOptionsValidationError {
    /// Create a new empty DbOptionsValidationError struct.
    pub fn new() -> Self {
        DbOptionsValidationError { fields: Vec::new() }
    }

    /// Insert an item into the DbOptionsValidationError list.
    ///
    /// # Arguments:
    ///
    /// * `field` - The field to insert.
    ///
    pub fn insert(&mut self, field: String) {
        self.fields.push(field);
    }

    /// Concatenate all the fields into a string.
    ///
    /// # Returns:
    ///
    /// * A comma delimited string containing all the invalid field names.
    ///
    pub fn fields_to_string(self) -> String {
        self.fields
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::DbOptions;

    #[test]
    fn test_construct_db_options___with_empty_props___returns_no_error() {
        assert!(DbOptions::new(HashMap::new()).is_ok());
    }

    #[test]
    fn test_construct_db_options___with_empty_props___returns_empty_options() {
        match DbOptions::new(HashMap::new()) {
            Ok(object) => {
                assert_eq!(None, object.unwrap_props());
            }
            Err(e) => {
                panic!(
                    "Error creating new DbOptions object: {}",
                    e.fields_to_string()
                );
            }
        }
    }

    #[test]
    fn test_construct_db_options___with_not_recognized_option___returns_error() {
        // self.assertRaises(DbOptionsValidationException, lambda: DbOptions({'wrong': 'option'}))
    }

    fn test_construct_db_options___with_option_base_path___raises_nothing_by_now() {
        // self.assertIsNotNone(DbOptions({'base_path': 'something'}))
    }
}
