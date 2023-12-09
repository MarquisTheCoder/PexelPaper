

<script lang="ts">
    export let wallpaper: string = "";


    function playWallpaper(){
       console.log("starting wallpaper");
    }

    function stopWallpaper(){
        console.log("stopping wallpaper");
    }
    
</script>

<div>
    <button>Play</button>
    <button>Stop</button>
</div>

<style>
    div{
        opacity: 0;
        position: absolute;
        top:0;
        left:0;
        height: 100%;
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 20px;
        justify-content: center;
        align-items: center;
        background-color: #00000011;
        backdrop-filter: blur(10px);
        transition-property: opacity;
        transition-duration: 1.3s;
    }

    div:hover{
        opacity: 1;
    }

    button{
        border-radius: 5px;
        color: #ffffff99;
        padding: 0 20px 0 20px;
        background-color: #1e407a;
        height: 45px;
        width: 60%;
    }
</style>