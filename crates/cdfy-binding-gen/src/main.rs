use fp_bindgen::{
    prelude::*, types::CargoDependency, BindingConfig, BindingsType, RustPluginConfig,
};
use once_cell::sync::Lazy;
use std::collections::{BTreeMap, BTreeSet};
use types::PluginMeta;

mod types;
use types::*;

fp_import! {
    fn rand() -> u32;

    fn debug(message: String);

    /// reserve task and execute returns `task_id`
    fn reserve(player_id: String, room_id: String, action: String, timeout: u32) -> String;
}

fp_export! {
    fn plugin_meta() -> PluginMeta;

    fn default_state() -> IResult;

    fn on_event(state: String, event: String) -> IResult;
}

static PLUGIN_DEPENDENCIES: Lazy<BTreeMap<&str, CargoDependency>> = Lazy::new(|| {
    BTreeMap::from([
        ("anyhow", CargoDependency::with_version("1")),
        (
            //このfp-bindgen-supportはほぼ必須
            "fp-bindgen-support",
            CargoDependency::with_version_and_features("2.0.0", BTreeSet::from(["async", "guest"])),
        ),
    ])
});

fn main() {
    for (bindings_type, path) in [
        // deno runtime
        // (
        //     BindingsType::TsRuntimeWithExtendedConfig(
        //         TsExtendedRuntimeConfig::new()
        //             .with_msgpack_module("https://unpkg.com/@msgpack/msgpack@2.7.2/mod.ts")
        //             .with_raw_export_wrappers(),
        //     ),
        //     "../../server/gen",
        // ),
        // wasmer runtime
        (BindingsType::RustWasmer2Runtime, "../cdfy-runtime/src/spec"),
        // rust plugin
        (
            BindingsType::RustPlugin(
                RustPluginConfig::builder()
                    .name("cdfy-sdk")
                    .version("0.1.0")
                    .dependencies(PLUGIN_DEPENDENCIES.clone())
                    .build(),
            ),
            "../cdfy-sdk",
        ),
    ] {
        let config = BindingConfig {
            bindings_type,
            path,
        };
        println!("generated {}", config.path);
        fp_bindgen!(config);
    }
}
