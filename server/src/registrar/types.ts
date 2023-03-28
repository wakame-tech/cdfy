export interface PluginEntity {
  id: string
  name: string
  version: string
  size: number
  createdAt: Date
  updatedAt: Date
}

export interface PluginPackage {
  meta: PluginEntity
  wasm: ArrayBuffer
}
