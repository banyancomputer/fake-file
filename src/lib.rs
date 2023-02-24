#![cfg_attr(docsrs, feature(doc_cfg))]
// #![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub, private_in_public)]

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

/// Crate utility functions
pub mod utils;

use crate::utils::fs_utils::create_random_file;

/// Enum for describing how to generate a file structure
/// This is used for generating random data for testing
#[derive(Serialize, Deserialize, Clone, Debug, strum::Display)]
pub enum Strategy {
    /// Generate exactly the file structure requested
    Simple,
    /// Generate a wide version of the file structure requested
    Wide,
    /// Generate a deep version of the file structure requested
    Deep,
    /// Generate a very shallow version of the file structure requested
    Directory,
    /// Generate one big file
    File,
}

/// Everything is a file in Unix :) including directories
/// Struct for representing a file structure, regardless of depth (i.e. a file or a directory)
/// We use this for generating random data for testing
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Structure {
    /// How many files should be in the file (if it has depth > 0)
    pub width: usize,
    /// How deep the directory structure should be. 0 means this is a file
    pub depth: usize,
    /// How much data should be in the file
    pub target_size: usize,
}

impl std::str::FromStr for Strategy {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Simple" => Ok(Strategy::Simple),
            "Wide" => Ok(Strategy::Wide),
            "Deep" => Ok(Strategy::Deep),
            "Directory" => Ok(Strategy::Directory),
            "File" => Ok(Strategy::File),
            _ => Err(format!("Invalid strategy: {}", s)),
        }
    }
}

impl Structure {
    /// Create a new FileStructure
    /// # Arguments
    /// width: Desired width of the file structure, upper bound
    /// depth: Desired depth of the file structure, upper bound
    /// target_size: Desired size of the file structure, upper bound
    /// strategy: The strategy to use for generating the file structure
    /// utf8_only: Whether or not files can include non-utf8 characters
    pub fn new(width: usize, depth: usize, target_size: usize, strategy: Strategy) -> Self {
        let s = match strategy {
            Strategy::Wide => {
                Structure {
                    width: width * 2,
                    depth: depth / 2,
                    target_size,
                }
            },
            Strategy::Deep => {
                Structure {
                    width: width / 2,
                    depth: depth * 2,
                    target_size,
                }
            },
            Strategy::Directory => {
                Structure {
                    width: 0,
                    depth: 0,
                    target_size,
                }
            },
            Strategy::File => {
                Structure {
                    width: 0,
                    depth: 0,
                    target_size,
                }
            },
            _ => {
                Structure {
                    width,
                    depth,
                    target_size,
                }
            }
        };
        s
    }

    /// Convert the FileStructure to a string that can be used as a filename
    /// # Example
    /// ```no_run
    /// use fake_file::{Structure, Strategy};
    /// let s = Structure::new(
    ///    4,                               // width
    ///   4,                               // depth
    ///  1024 * 1024,
    /// Strategy::Simple                   // target size in bytes (1Mb)
    /// );
    /// assert_eq!(s.to_path_string(), "w4_d4_s1048576");
    /// ```
    pub fn to_path_string(&self) -> String {
        format!(
            "w{}_d{}_s{}",
            self.width, self.depth, self.target_size
        )
    }

    /// Generate a FileStructure with the given path. Does not check if the path can hold
    /// the file structure. Use with caution!
    /// # Arguments
    /// path: The path to generate the file structure at
    /// # Panics
    /// Panics if it cant create a directory at the given path (i.e. the path parent doesn't exist)
    /// Panics if the path already exists
    /// Errors if the file structure cannot be generated
    pub fn generate(&self, path: &Path) -> Result<(), Error> {
        // Panic if the path already exists. We don't want to overwrite anything!
        assert!(!path.exists());
        // If this is 0, we're creating a file
        if self.depth == 0 {
            // let file_path = path;
            // // Create a file with the target size

            create_random_file(path, self.target_size);
            return Ok(()); // We're done here
        }
        fs::create_dir(path).unwrap();
        for i in 0..self.width {
            // Read a fixed amount of data from target size
            let target_size = self.target_size / self.width;
            // Push the new path onto the path
            // let mut new_path = path.clone();
            let new_path = path.join(i.to_string());
            // Generate a new FileStructure with the new path
            Structure::new(
                self.width,
                self.depth - 1,
                target_size,
                Strategy::Simple,
            )
                .generate(&new_path)
                .unwrap();
        }
        Ok(())
    }
}
