import { createRuntime, Exports } from './gen/index.ts'

const plugins: Record<string, Exports> = {}

export const registerPluginFromLocal = async (path: string): Promise<void> => {
  const imports = {}
  const plugin = await Deno.readFile(path)
  const runtime = await createRuntime(plugin, imports)
  const meta = runtime.pluginMeta?.()!
  console.log(`installed ${meta.name} v${meta.version} @ ${path}`)
  plugins[meta.name] = runtime
}

export const getPlugin = (name: string): Exports | undefined => {
  return plugins[name]
}
