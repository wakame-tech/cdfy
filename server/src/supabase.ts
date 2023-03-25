import { createClient } from 'https://deno.land/x/supabase@1.3.1/mod.ts'

const projectUrl = Deno.env.get('SUPABASE_PROJECT_URL')!
const apiKey = Deno.env.get('SUPABASE_API_KEY')!
export const supabase = createClient(projectUrl, apiKey)
