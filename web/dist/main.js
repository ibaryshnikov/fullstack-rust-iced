import init from './pkg/web.js';

window.addEventListener('load', async () => {
    console.log("Window loaded");
    await init();
});
