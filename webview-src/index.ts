import { invoke } from '@tauri-apps/api/core'

export interface Browser {
  name: string,
  last_sync: number,
}

export interface Config {
  browsers: Array<Browser>
}

export interface History {
  title: string,
  url: string,
  last_visited: number,
}

export async function getConfig() {
  return await invoke<Config>('plugin:shion-history|get_config')
}

export async function setConfig(config: Config) {
  return await invoke('plugin:shion-history|set_config', {
    config
  })
}

export async function readHistory(name: string, start: number, end: number) {
  return await invoke<Array<History>>('plugin:shion-history|read_history', {
    name,
    start,
    end
  })
}
