
use toml_edit::{Document, value, Array};

use std::fs::{read_to_string, OpenOptions};
use ansi_term::*;
use std::process::exit;
use std::io::Write;
use std::collections::HashMap;

/// The heart of your spec
pub struct Program {
    name: &'static str,
    authors: Array,
    dependencies: HashMap<&'static str, &'static str>,
    version: &'static str,
    edition: &'static str
}


impl Program {
    /// Creates a new program struct.
    ///
    /// **All values are blank**
    pub fn new() -> Program {
        Program {
            // 0000 means required value not set
            // blank means optional
            name: "placeholder name",
            authors: Array::default(),
            version: "plaeceholder",
            edition: "2018",
            dependencies: HashMap::new()
        }
    }

    pub fn edition(mut self, editon: &'static str)  -> Self{
        info("With Edition");
        info(editon);
        self.edition  = editon;
        self
    }

    /// Adds an author
    ///
    /// **It is required**
    pub fn author(mut self ,author: &'static str)  -> Self{
        info("Author name is");
        info(author);
        self.authors.push(author);
        self
    }
    /// Sets the version
    ///
    /// **It is required**
    pub fn version(mut self, version: &'static str)  -> Self{
        info("Package Version is");
        info(version);
        self.version = version;
        self
    }
    /// Checks the package for invalid config
    pub fn check(&self) {
        if self.name == "placeholder name" || self.authors.is_empty() || self.version == "placeholder"  {
            error(" Missing required fields. Name, Author and Version are required in spec.rs");
        }

    }

    /// Adds a name propety under  the package section
    ///
    /// Names are **required**
    pub fn name(  mut self,name: &'static str)  -> Self{
        info("Package name is");
        info(name);
        self.name = name;

        self
    }
    /// Adds a dependencie
    pub fn dependencie(mut self, name: &'static str, version: &'static str) -> Self {
        info("Dependencie");
        info(name);
        info("With version");
        info(version);
        self.dependencies.insert(name, version);
        self
    }
    /// Generates cargo toml.
    ///
    /// __Also runs the `check` function__
    pub fn gen(self) {
        good("Generating....");
        self.check();
        let  cargo_toml = read_to_string("Cargo.toml");
        if let Result::Ok(cargo_toml) = cargo_toml {
            let  cargo_toml_doc = cargo_toml.parse::<Document>();
            if let Ok( mut tomlspec) = cargo_toml_doc {
                tomlspec["package"]["name"] = value(self.name);
                tomlspec["package"]["authors"] = value(self.authors);
                tomlspec["package"]["version"] = value(self.version);
                tomlspec["package"]["edition"] = value(self.edition);

                for dep in self.dependencies {
                    tomlspec["dependencies"][dep.0] = value(dep.1)
                }
                let mut cargo_file = OpenOptions::new().write(true).open("Cargo.toml")
                .expect("Fatal error. Cargo toml not found. This is probably a bug. Please report it on github.");
                cargo_file.write(tomlspec.to_string().as_bytes()).expect("Cargo.toml erorr");
            }
        } else {
            error(" Previous Cargo.toml is invalid. Run cargo check to see the mistakes")

        }

    }
}


fn info (reason: &str) {
    eprintln!("{} {}", ansi_term::Color::Blue.bold().paint("Info:"), Color::Cyan.paint(reason) );

}

fn good (reason: &str) {
    eprintln!("{} {}", ansi_term::Color::Green.bold().paint("Good news!"), Color::Cyan.paint(reason) );

}

fn error (reason: &str) {
eprintln!("{}: {}", ansi_term::Color::Red.bold().paint("Whoops an error has occured"), Color::Cyan.paint(reason) );
exit(1);
}
