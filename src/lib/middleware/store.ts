// import os from "os";

import { invoke } from '@tauri-apps/api/tauri';
import { writable } from "svelte/store";


let current_host_username = await invoke('current_user');
export let default_wallpaper_path = `/Users/${current_host_username}/Movies`;

const storeSchema: Object = {}

export let wallpaper_store = writable([storeSchema]);

export const current_path = writable(default_wallpaper_path);

