<script>
      import { appWindow } from "@tauri-apps/api/window"
</script>

<div data-tauri-drag-region class="titlebar rounded-sm">
    <div role="button" tabindex="0" class="titlebar-button" id="titlebar-minimize" on:keypress on:click={() => appWindow.minimize()}>
        <img src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize" />
    </div>
    <div role="button" tabindex="0" class="titlebar-button h-10" id="titlebar-maximize" on:keypress on:click={() => appWindow.toggleMaximize()}>
        <img src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize" />
    </div>
    <div role="button" tabindex="0" class="titlebar-button" id="titlebar-close" on:keypress on:click={() => appWindow.close()}>
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </div>
</div>



<style>
    .titlebar {
  height: 30px;
  background: #062c35;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}
</style>