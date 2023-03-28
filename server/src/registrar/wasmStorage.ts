import { SupabaseClient } from '../deps.ts'

export interface WasmStorage {
  uploadWasm(key: string, wasm: ArrayBuffer): Promise<void>
  downloadWasm(key: string): Promise<ArrayBuffer>
}

export class SupabaseWasmStorage implements WasmStorage {
  constructor(private supabase: SupabaseClient) {}

  async downloadWasm(key: string): Promise<ArrayBuffer> {
    const res = await this.supabase.storage.from('plugins').download(key)
    if (res.error) {
      throw res.error
    }
    const blob = res.data!
    return blob.arrayBuffer()
  }

  async uploadWasm(key: string, wasm: ArrayBuffer): Promise<void> {
    const res = await this.supabase.storage.from('plugins').upload(key, wasm, {
      upsert: true,
      contentType: 'application/wasm',
    })
    if (res.error) {
      throw res.error
    }
  }
}

export class LocalWasmStorage implements WasmStorage {
  downloadWasm(key: string): Promise<ArrayBuffer> {
    return Deno.readFile(`./_registry/${key}.wasm`)
  }

  async uploadWasm(key: string, wasm: ArrayBuffer): Promise<void> {
    await Deno.writeFile(`./_registry/${key}.wasm`, new Uint8Array(wasm))
  }
}
