




<section class="bg-black">
    <div id="background"></div>
    <slot/>
</section>

<style>
    
    #background{
        position: absolute;
        height: 100vh;
        width: 100vw    ;
        top: 0;
        left: 0;
        z-index: -1;
        background: radial-gradient(circle at center, #0e1018, #050508);
        backdrop-filter: blur(100px);
        border-radius: 10px;
    }
    
    section{
        width: 100vw;
        height: 100vh;
        background-color: #1A1C25;
        display: flex;
        flex-direction: column;
        margin: 0;
        padding-top: 30px;
        z-index: 2;
        position: relative;
        border-radius: 10px;
    }
</style>