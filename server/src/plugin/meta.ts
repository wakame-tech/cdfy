import { PluginMeta, SupabaseClient } from '../deps.ts'

export interface PluginEntity {
  id: string
  name: string
  version: string
  size: number
  createdAt: Date
  updatedAt: Date
}

export const savePluginMeta = async (
  supabase: SupabaseClient,
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

export const getPluginMetas = async (
  supabase: SupabaseClient
): Promise<PluginEntity[]> => {
  const { data, error } = await supabase.from('plugin_metas')
  if (error) {
    throw error
  }
  return data
}
