import { getPluginMetas } from '../src/plugin/meta.ts'
import { getPluginPackage } from '../src/plugin/registry.ts'
import { supabase } from '../src/supabase.ts'

console.log(await getPluginMetas(supabase))

// const plugin = await registerPlugin(supabase, './counter.wasm')
const id = 'd8e30e24-4b98-454a-82be-6e49233ebc33'
const pkg = await getPluginPackage(supabase, id)
console.log(pkg.wasm.byteLength)
