import { invoke } from '@tauri-apps/api/core'

interface Browser {
  name: string,
  last_sync: number,
}

interface Config {
  browsers: Array<Browser>
}

export async function getConfig() {
  return await invoke<Config>('plugin:shion-history|get_config')
}
