// import os from "os";

import { writable } from "svelte/store";

let current_host_username = "";

// async function current_user(){
//     let current_host_username = await invoke('current_user');
// }

let default_wallpaper_path = `/Users/${current_host_username}/Movies`;
export const current_path = writable(default_wallpaper_path);
