import { SupabaseClient } from '../deps.ts'

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
