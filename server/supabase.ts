import {
  createClient,
  SupabaseClient,
} from 'https://deno.land/x/supabase@1.3.1/mod.ts'
import { PluginMeta } from './gen/types.ts'
import { instantiate } from './plugin.ts'
const projectUrl = Deno.env.get('SUPABASE_PROJECT_URL')!
const apiKey = Deno.env.get('SUPABASE_API_KEY')!
const supabase = createClient(projectUrl, apiKey)

export interface PluginEntity {
  id: string
  name: string
  version: string
  size: number
  createdAt: Date
  updatedAt: Date
}

export const registerPlugin = async (
  supabase: SupabaseClient,
  path: string
): Promise<PluginEntity> => {
  const wasm = await Deno.readFile(path)
  const instance = await instantiate(wasm)
  const meta = instance.pluginMeta?.()!
  const pluginEntity = await savePluginMeta(meta, wasm.byteLength)
  await uploadWasm(supabase, pluginEntity.id, wasm)
  return pluginEntity
}

export const getPlugin = async (
  supabase: SupabaseClient,
  id: string
): Promise<ArrayBuffer> => {
  const pluginEntity = await getPluginMeta(supabase, id)
  console.log(pluginEntity)
  const wasm = await downloadWasm(supabase, pluginEntity.id)
  return wasm
}

export const savePluginMeta = async (
  meta: PluginMeta,
  size: number
): Promise<PluginEntity> => {
  const plugin: PluginEntity = {
    id: crypto.randomUUID(),
    name: meta.name,
    version: meta.version,
    size,
    createdAt: new Date(),
    updatedAt: new Date(),
  }
  console.log(plugin)
  const { error } = await supabase.from('plugin_metas').insert(plugin)
  if (error) {
    throw error
  }
  return plugin
}

export const getPluginMeta = async (
  supabase: SupabaseClient,
  id: string
): Promise<PluginEntity> => {
  const { data, error } = await supabase
    .from('plugin_metas')
    .select()
    .eq('id', id)
  if (error) {
    throw error
  }
  if (data.length !== 1) {
    throw 'not found'
  }
  return data[0]
}

export const uploadWasm = async (
  supabase: SupabaseClient,
  key: string,
  wasm: ArrayBuffer
): Promise<void> => {
  const res = await supabase.storage.from('plugins').upload(key, wasm, {
    upsert: true,
    contentType: 'application/wasm',
  })
  if (res.error) {
    throw res.error
  }
}

export const downloadWasm = async (
  supabase: SupabaseClient,
  key: string
): Promise<ArrayBuffer> => {
  const res = await supabase.storage.from('plugins').download(key)
  if (res.error) {
    throw res.error
  }
  const blob = res.data!
  return blob.arrayBuffer()
}

// const plugin = await registerPlugin(supabase, './counter.wasm')
const id = 'd8e30e24-4b98-454a-82be-6e49233ebc33'
const wasm = await getPlugin(supabase, id)
console.log(wasm.byteLength)
