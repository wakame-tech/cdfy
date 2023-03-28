import { Command } from 'https://deno.land/x/cliffy@v0.19.2/command/mod.ts'
import { LocalPluginMetaRepository } from './registrar/PluginMetaRepository.ts'
import { PluginRegistrarService } from './registrar/PluginRegistrarService.ts'
import { LocalWasmStorage } from './registrar/wasmStorage.ts'

const service = new PluginRegistrarService(
  new LocalPluginMetaRepository(),
  new LocalWasmStorage()
)

const { args } = await new Command()
  .name('registrar')
  .version('0.1.0')
  .arguments('<arg>')
  .parse(Deno.args)

const path = args[0]
const entity = await service.registerPackage(path)
console.log(entity)

const pkgs = await service.findPackages()
console.log(`total ${pkgs.length} packages found`)
console.log(
  pkgs.map((pkg) => `${pkg.name} v${pkg.version} ${pkg.size} B`).join('\n')
)
