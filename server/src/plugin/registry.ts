import { SupabaseClient } from '../deps.ts'
import { PluginEntity, getPluginMeta, savePluginMeta } from './meta.ts'
import { instantiate } from './runtime.ts'
import { downloadWasm, uploadWasm } from './wasmStorage.ts'

export interface PluginPackage {
  meta: PluginEntity
  wasm: ArrayBuffer
}

export const registerPackageFromLocal = async (
  supabase: SupabaseClient,
  path: string
): Promise<PluginEntity> => {
  const wasm = await Deno.readFile(path)
  const instance = await instantiate(wasm)
  const meta = instance.pluginMeta?.()!
  const pluginEntity = await savePluginMeta(supabase, meta, wasm.byteLength)
  await uploadWasm(supabase, pluginEntity.id, wasm)
  return pluginEntity
}

export const getPluginPackage = async (
  supabase: SupabaseClient,
  id: string
): Promise<PluginPackage> => {
  const meta = await getPluginMeta(supabase, id)
  console.log(meta)
  const wasm = await downloadWasm(supabase, meta.id)
  return {
    wasm,
    meta,
  }
}

export const getPluginPackageFromLocal = async (
  path: string
): Promise<PluginPackage> => {
  const wasm = await Deno.readFile(path)
  const meta: PluginEntity = {
    id: 'counter',
    name: path,
    version: '1',
    size: wasm.byteLength,
    createdAt: new Date(),
    updatedAt: new Date(),
  }
  console.debug(meta)
  return {
    wasm,
    meta,
  }
}
