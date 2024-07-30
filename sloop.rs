use std::{
    io::{self, Write},
    path::Path,
    process::Command,
};

mod term {
    pub fn colored(r: i32, g: i32, b: i32, text: &str) -> String {
        return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
    }
}

#[derive(Clone, Debug)]
pub struct Dependency {
    crate_name: String,
    rlib_path: String,
}

#[derive(Clone)]
pub struct DependencyBuilder {
    crate_name: String,
    path: String,
    proc_macro: bool,
    edition: String,
    dependencies: Vec<Dependency>,
    features: Vec<String>,
    optimization: i32,
}

impl DependencyBuilder {
    pub fn new(path: &str) -> DependencyBuilder {
        let crate_name = Path::new(path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace("-", "_");

        DependencyBuilder {
            crate_name,
            path: path.to_owned(),
            proc_macro: false,
            edition: "2021".to_string(),
            dependencies: vec![],
            features: vec![],
            optimization: 0,
        }
    }

    pub fn with_dependency(self, dependency: &Dependency) -> DependencyBuilder {
        let mut dependencies = self.dependencies;
        dependencies.push(dependency.clone());
        DependencyBuilder {
            dependencies,
            ..self
        }
    }

    pub fn with_feature(self, feature: &str) -> DependencyBuilder {
        let mut features = self.features;
        features.push(feature.to_string());
        DependencyBuilder { features, ..self }
    }

    pub fn edition(self, edition: &str) -> DependencyBuilder {
        DependencyBuilder {
            edition: edition.to_string(),
            ..self
        }
    }

    pub fn crate_name(self, crate_name: &str) -> DependencyBuilder {
        DependencyBuilder {
            crate_name: crate_name.to_string(),
            ..self
        }
    }

    pub fn proc_macro(self, proc_macro: bool) -> DependencyBuilder {
        DependencyBuilder { proc_macro, ..self }
    }

    pub fn optimization(self, optimization: i32) -> DependencyBuilder {
        DependencyBuilder {
            optimization,
            ..self
        }
    }

    pub fn build(self) -> Result<Dependency, ()> {
        let rlib_path = format!(
            ".sloop/lib{}.{}",
            self.crate_name,
            if self.proc_macro { "so" } else { "rlib" }
        );
        if Path::new(&rlib_path).exists() {
            println!(
                "{} is up to date",
                term::colored(155, 255, 155, &self.crate_name)
            );
            return Ok(Dependency {
                rlib_path: rlib_path.to_string(),
                crate_name: self.crate_name,
            });
        }
        println!(
            "Building {}",
            term::colored(155, 255, 155, &self.crate_name)
        );
        let mut cmd = Command::new("rustc");
        cmd.arg("--color").arg("always");
        if self.proc_macro {
            cmd.arg("--crate-type").arg("proc-macro");
            cmd.arg("--extern").arg("proc_macro");
        } else {
            cmd.arg("--crate-type").arg("lib");
        }
        cmd.arg("--crate-name").arg(&self.crate_name);
        cmd.arg("--edition").arg(&self.edition);
        cmd.arg("--out-dir").arg(".sloop");
        cmd.arg(&format!("{}/src/lib.rs", self.path));
        for dependency in &self.dependencies {
            cmd.arg("--extern");
            cmd.arg(&format!(
                "{}={}",
                dependency.crate_name, dependency.rlib_path
            ));
        }
        for feature in &self.features {
            cmd.arg("--cfg");
            cmd.arg(&format!("feature=\"{}\"", feature));
        }
        cmd.arg("-L").arg(".sloop");
        cmd.arg("-C").arg(&format!("opt-level={}", self.optimization));
        let output = cmd.output().unwrap();

        if !output.status.success() {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            println!("{cmd:?}");
            return Err(());
        }
        Ok(Dependency {
            rlib_path: rlib_path.to_string(),
            crate_name: self.crate_name,
        })
    }
}

pub struct Builder {
    binary: bool,
    library: bool,
    name: Option<String>,
    entrypoint: String,
    edition: String,
    dependencies: Vec<Dependency>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            binary: false,
            library: false,
            name: None,
            entrypoint: "src/main.rs".to_string(),
            edition: "2018".to_string(),
            dependencies: vec![],
        }
    }

    pub fn binary(self) -> Builder {
        Builder {
            binary: true,
            ..self
        }
    }

    pub fn name(self, name: &str) -> Builder {
        Builder {
            name: Some(name.to_owned()),
            ..self
        }
    }

    pub fn entrypoint(self, entrypoint: &str) -> Builder {
        Builder {
            entrypoint: entrypoint.to_owned(),
            ..self
        }
    }

    pub fn with_dependency(self, dependency: &Dependency) -> Builder {
        let mut dependencies = self.dependencies;
        dependencies.push(dependency.clone());
        Builder {
            dependencies,
            ..self
        }
    }

    pub fn edition(self, edition: &str) -> Builder {
        Builder {
            edition: edition.to_string(),
            ..self
        }
    }

    pub fn build(self) -> Result<(), ()> {
        println!(
            "Building {} {}",
            term::colored(155, 155, 255, self.name.as_deref().unwrap_or("")),
            self.entrypoint
        );
        if self.library {
            unimplemented!();
        }

        let mut cmd = Command::new("rustc");
        cmd.arg("--color").arg("always");
        if self.binary {
            cmd.arg("--crate-type").arg("bin");
        }
        if let Some(name) = self.name {
            cmd.arg("--crate-name").arg(name);
        }
        cmd.arg("--edition").arg(&self.edition);
        for dependency in &self.dependencies {
            cmd.arg("--extern");
            cmd.arg(&format!(
                "{}={}",
                dependency.crate_name, dependency.rlib_path
            ));
        }
        cmd.arg("-L").arg(".sloop");
        cmd.arg(&self.entrypoint);
        let output = cmd.output().unwrap();
        if !output.status.success() {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            println!("{cmd:?}");
            Err(())
        } else {
            println!("Done!");
            Ok(())
        }
    }
}
