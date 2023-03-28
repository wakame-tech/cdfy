import { PluginMetaRepository } from './PluginMetaRepository.ts'
import { getMeta } from '../runtime/runtime.ts'
import { WasmStorage } from './wasmStorage.ts'
import { PluginEntity, PluginPackage } from './types.ts'

export interface IPluginRegistrarService {
  registerPackage(path: string): Promise<PluginEntity>
  findPackages(): Promise<PluginEntity[]>
  fetchPackage(id: string): Promise<PluginPackage>
}

export const registerLocalPackages = async (
  pluginMetaRepository: PluginMetaRepository
): Promise<void> => {
  for await (const path of Deno.readDir('./_registry')) {
    if (path.isFile && path.name.endsWith('.wasm')) {
      const uuid = path.name.split('.')[0]
      const wasm = await Deno.readFile(`./_registry/${path.name}`)
      const meta = await getMeta(wasm)
      if (!meta) {
        console.error('metadata not found')
        continue
      }
      const entity: PluginEntity = {
        id: uuid,
        name: meta.name,
        version: meta.version,
        size: wasm.byteLength,
        createdAt: new Date(),
        updatedAt: new Date(),
      }
      await pluginMetaRepository.save(entity)
      console.log(`register ${entity.name}@v${entity.version} (${entity.id})`)
    }
  }
}

export class PluginRegistrarService implements IPluginRegistrarService {
  constructor(
    private pluginMetaRepository: PluginMetaRepository,
    private wasmStorage: WasmStorage
  ) {}

  async registerPackage(path: string): Promise<PluginEntity> {
    const wasm = await Deno.readFile(path)
    const meta = await getMeta(wasm)
    if (!meta) {
      throw 'metadata not found'
    }
    const entity: PluginEntity = {
      id: crypto.randomUUID(),
      name: meta.name,
      version: meta.version,
      size: wasm.byteLength,
      createdAt: new Date(),
      updatedAt: new Date(),
    }
    const pluginEntity = await this.pluginMetaRepository.save(entity)
    await this.wasmStorage.uploadWasm(pluginEntity.id, wasm)
    return pluginEntity
  }

  findPackages(): Promise<PluginEntity[]> {
    return this.pluginMetaRepository.getAll()
  }

  async fetchPackage(id: string): Promise<PluginPackage> {
    const meta = await this.pluginMetaRepository.get(id)
    console.log(meta)
    const wasm = await this.wasmStorage.downloadWasm(meta.id)
    return {
      wasm,
      meta,
    }
  }
}
