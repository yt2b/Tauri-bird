import("../pkg/index.js").catch(console.error);
import { appWindow } from "@tauri-apps/api/window"

const setCanvasResolution = async () => {
    const canvas = document.getElementById("canvas");
    const size = await appWindow.innerSize();
    canvas.width = size.width;
    canvas.height = size.height;
    const ratio = window.devicePixelRatio;
    canvas.style.width = (size.width / ratio) + "px";
    canvas.style.height = (size.height / ratio) + "px";
    canvas.getContext("2d").scale(ratio, ratio);
};
setCanvasResolution();
