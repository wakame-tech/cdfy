use fp_bindgen::{
    prelude::*, types::CargoDependency, BindingConfig, BindingsType, RustPluginConfig,
};
use once_cell::sync::Lazy;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Serializable)]
pub struct State {
    pub data: String,
}

#[derive(Serializable)]
pub struct PluginMeta {
    pub name: String,
    pub version: String,
}

fp_import! {
    fn rand() -> u32;
}

fp_export! {
    /// returns plugin meta infomation
    fn plugin_meta() -> PluginMeta;

    /// fire when a room is created
    fn on_create_room(player_id: String) -> State;

    /// fire when join a player
    fn on_join_player(player_id: String, state: State) -> State;

    /// fire when leave a player
    fn on_leave_player(player_id: String, state: State) -> State;

    /// fire when elements clicked
    fn on_click(player_id: String, id: String, state: State, value: String) -> State;
}

static PLUGIN_DEPENDENCIES: Lazy<BTreeMap<&str, CargoDependency>> = Lazy::new(|| {
    BTreeMap::from([
        // (
        //     //生成プラグインで必須なcrateを設定
        //     "regex",
        //     CargoDependency {
        //         version: Some("1.6.0"),
        //         ..CargoDependency::default()
        //     },
        // ),
        (
            //このfp-bindgen-supportはほぼ必須
            "fp-bindgen-support",
            CargoDependency::with_version_and_features("2.0.0", BTreeSet::from(["async", "guest"])),
        ),
    ])
});

fn main() {
    for (bindings_type, path) in [
        // rust wasmer runtime
        // (BindingsType::RustWasmerRuntime, "../runtime/src/gen"),
        // deno runtime
        (
            BindingsType::TsRuntimeWithExtendedConfig(
                TsExtendedRuntimeConfig::new()
                    .with_msgpack_module("https://unpkg.com/@msgpack/msgpack@2.7.2/mod.ts")
                    .with_raw_export_wrappers(),
            ),
            "../../server/gen",
        ),
        // rust plugin
        (
            BindingsType::RustPlugin(RustPluginConfig {
                name: "cdfy-sdk",
                authors: "[\"wakame-tech\"]",
                version: "0.1.0",
                dependencies: PLUGIN_DEPENDENCIES.clone(),
            }),
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
