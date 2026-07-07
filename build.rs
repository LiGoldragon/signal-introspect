use std::{env, path::PathBuf};

use schema_rust::build::{DependencySchema, GenerationDriver, GenerationPlan};

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/lib.schema");
        println!("cargo:rerun-if-changed=src/schema/lib.rs");
        println!("cargo:rerun-if-env-changed=DEP_SIGNAL_PERSONA_SCHEMA_DIR");
        println!("cargo:rerun-if-env-changed=DEP_SIGNAL_MESSAGE_SCHEMA_DIR");
        println!(
            "cargo::metadata=schema-dir={}",
            self.crate_root.join("schema").display()
        );

        let persona_schema =
            DependencySchema::from_cargo_metadata("signal-persona", "signal-persona", "0.2.0")
                .expect("read signal-persona schema metadata")
                .expect(
                    "signal-persona schema directory exposed via DEP_SIGNAL_PERSONA_SCHEMA_DIR",
                );
        let message_schema =
            DependencySchema::from_cargo_metadata("signal-message", "signal-message", "0.3.0")
                .expect("read signal-message schema metadata")
                .expect(
                    "signal-message schema directory exposed via DEP_SIGNAL_MESSAGE_SCHEMA_DIR",
                );

        GenerationDriver::new(
            GenerationPlan::wire_contract(&self.crate_root, "signal-introspect", "0.1.0")
                .with_dependency_schema(persona_schema)
                .with_dependency_schema(message_schema),
        )
        .generate()
        .expect("generate signal-introspect schema artifacts")
        .write_or_check("SIGNAL_INTROSPECT_UPDATE_SCHEMA_ARTIFACTS")
        .expect("checked-in signal-introspect schema artifacts are fresh");
    }
}
