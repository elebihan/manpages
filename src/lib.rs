//
// This file is part of manpages
//
// Copyright (c) 2017 Eric Le Bihan
//
// Licensed under MIT license <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to
// those terms.
//

//! A library for building MAN pages
//!
//! This library is intended to be used as a `build-dependencies` entry in
//! `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! manpages = "0.1"
//! ```
//!
//! The purpose of this crate is to provide the utility functions necessary to
//! build MAN pages from Markdown of DocBooks documents.
//! The conversion is done using [Pandoc](http://pandoc.org)
//! and [xlstproc](http://xmlsoft.org/XSLT/xsltproc2.html).
//!
//! # Example
//!
//! Add the following code to build MAN pages from the ``man`` directory:
//!
//! ```no_run
//! extern crate manpages;
//!
//! use std::env;
//! use std::path::PathBuf;
//! use manpages::build;
//!
//! fn main() {
//!     let mut dst_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
//!     dst_dir.push("man");
//!     build("man", &dst_dir).ok().expect("Failed to build MAN pages");
//! }
//! ```

extern crate regex;

use std::ffi::OsStr;
use std::fs::{create_dir_all, read_dir};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use regex::Regex;

fn run_command(cmd: &mut Command) -> Result<()> {
    println!("Running {:?}", cmd);
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Command failed"))
    }
}

fn run_pandoc<P: AsRef<Path> + AsRef<OsStr>>(input: P, output: P) -> Result<()> {
    let mut cmd = Command::new("pandoc");
    cmd.arg("-s");
    cmd.arg("-f");
    cmd.arg("markdown");
    cmd.arg("-t");
    cmd.arg("man");
    cmd.arg("-o");
    cmd.arg(output);
    cmd.arg(input);
    run_command(&mut cmd)
}

fn run_xsltproc<P: AsRef<Path> + AsRef<OsStr>>(input: P, output: P) -> Result<()> {
    let mut cmd = Command::new("xsltproc");
    cmd.arg("--nonet");
    cmd.arg("--stringparam");
    cmd.arg("man.output.quietly");
    cmd.arg("1");
    cmd.arg("--param");
    cmd.arg("funcsynopsis.style");
    cmd.arg("'ansi'");
    cmd.arg("--output");
    cmd.arg(output);
    cmd.arg(
        "http://docbook.sourceforge.net/release/xsl/current/manpages/docbook.xsl",
    );
    cmd.arg(input);
    run_command(&mut cmd)
}

/// Build MAN pages from documents in `src_dir` to `dst_dir` directory.
///
/// Any file ending with `*.md` or `*.xml` in `src_dir` will be converted.
pub fn build<P: AsRef<Path>, Q: AsRef<Path>>(src_dir: P, dst_dir: Q) -> Result<()> {

    let re = Regex::new(r"(?:.+)\.([0-9])\.(md|xml)").unwrap();

    for entry in read_dir(src_dir)? {
        let entry = entry?;
        let input = entry.path();
        if let Some(caps) = re.captures(input.to_str().unwrap()) {
            let basename = input.file_stem().unwrap();
            let man_dir = caps.get(1).unwrap().as_str();
            let ext = caps.get(2).unwrap().as_str();
            let mut output = PathBuf::new();
            output.push(&dst_dir);
            output.push(&man_dir);
            create_dir_all(&output)?;
            output.push(basename);
            let result = match ext {
                "md" => run_pandoc(&input, &output),
                "xml" => run_xsltproc(&input, &output),
                _ => Ok(()),
            };
            result?;
            println!("cargo:rerun-if-changed={}", input.display());
        }
    }

    Ok(())
}
