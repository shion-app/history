import { invoke } from '@tauri-apps/api/core'

interface Browser {
  name: string,
  last_sync: number,
}

interface Config {
  browsers: Array<Browser>
}

interface History {
  title: string,
  url: string,
  last_visited: number,
}

export async function getConfig() {
  return await invoke<Config>('plugin:shion-history|get_config')
}

export async function readHistory(list: Array<string>, start: number, end: number) {
  return await invoke<Array<History>>('plugin:shion-history|read_history', {
    list,
    start,
    end
  })
}
