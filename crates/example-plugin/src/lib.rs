use plugin_bindings::{fp_export_impl, Data};

#[fp_export_impl(plugin_bindings)]
pub fn data_check(data: Data) -> u32 {
    (data.name.len() > 0) as u32
}
