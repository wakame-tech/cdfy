import { SupabaseClient } from '../deps.ts'
import { PluginEntity } from './types.ts'

export interface PluginMetaRepository {
  save(meta: PluginEntity): Promise<PluginEntity>
  get(id: string): Promise<PluginEntity>
  getAll(): Promise<PluginEntity[]>
}

export class SupabasePluginMetaRepository implements PluginMetaRepository {
  constructor(private supabase: SupabaseClient) {}

  async save(meta: PluginEntity): Promise<PluginEntity> {
    const { error } = await this.supabase.from('plugin_metas').insert(meta)
    if (error) {
      throw error
    }
    return meta
  }

  async get(id: string): Promise<PluginEntity> {
    const { data, error } = await this.supabase
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

  async getAll(): Promise<PluginEntity[]> {
    const { data, error } = await this.supabase.from('plugin_metas')
    if (error) {
      throw error
    }
    return data
  }
}

export class LocalPluginMetaRepository implements PluginMetaRepository {
  metas: Record<string, PluginEntity>

  constructor() {
    this.metas = {}
  }

  save(meta: PluginEntity): Promise<PluginEntity> {
    this.metas[meta.id] = meta
    return Promise.resolve(meta)
  }

  get(id: string): Promise<PluginEntity> {
    if (!this.metas[id]) {
      throw `plugin ${id} not found`
    }
    return Promise.resolve(this.metas[id])
  }

  getAll(): Promise<PluginEntity[]> {
    return Promise.resolve(Object.values(this.metas))
  }
}
