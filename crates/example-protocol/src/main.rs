use fp_bindgen::{
    prelude::*, types::CargoDependency, BindingConfig, BindingsType, RustPluginConfig,
};
use once_cell::sync::Lazy;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Serializable)]
pub struct Data {
    pub name: String,
    pub text: String,
}

fp_import! {}

fp_export! {
    fn data_check(data: Data) -> u32;
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
        // host側へのコード生成
        (BindingsType::RustWasmerRuntime, "../runtime/src/gen"),
        // plugin側へのコード生成
        (
            BindingsType::RustPlugin(RustPluginConfig {
                name: "plugin-bindings",
                authors: r#"["aobat"]"#,
                version: "0.1.0",
                dependencies: PLUGIN_DEPENDENCIES.clone(),
            }),
            "../plugin-bindings",
        ),
    ] {
        let config = BindingConfig {
            bindings_type,
            path,
        };
        println!("{:?}", config);
        fp_bindgen!(config);
    }
}
