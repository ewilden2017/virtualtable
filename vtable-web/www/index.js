import * as wasm from "vtable-web";
import { memory } from "vtable-web/vtable_web_bg";
import {fps} from "./fps.js";

const TOKEN_STYLES = ["#AA0000", "#00AA00"];
const TOKEN_RADIUS = 20;

// Initialize canvas object.
const canvas = document.getElementById('maincanvas');
const ctx = canvas.getContext('2d');

// Set up a token to move around.
const mainscene = wasm.Scene.new();
const token1 = wasm.TokenReference.new(50, 50, TOKEN_RADIUS, 0);
const token2 = wasm.TokenReference.new(200, 50, TOKEN_RADIUS, 1);

mainscene.add_token(token1);
mainscene.add_token(token2);

const fps_counter = new fps();
const fps_display = document.getElementById("fps");

// Drawing Functions
//
function clear() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
}

function drawToken(token) {
    ctx.beginPath();
    ctx.arc(token.get_x(), token.get_y(), token.get_radius(), 0, 2 * Math.PI, false);
    ctx.fillStyle = TOKEN_STYLES[token.get_style()];
    ctx.fill();
    ctx.lineWidth = 1;
    ctx.strokeStyle = "#000000";
    ctx.stroke();
}

var updatePending = false;
const updateCanvas = () => {
    const ms_frame = fps_counter.update();
    const avg_fps = fps_counter.get_fps();
    fps_display.textContent = `FPS: ${Math.round(avg_fps)} (${ms_frame} ms)`;

    updatePending = false;
    clear();

    mainscene.each_token(drawToken);
}

// Wrapper function to make sure only one update is requested per frame.
const scheduleUpdate = () => {
    if (!updatePending) {
        updatePending = true;
        requestAnimationFrame(updateCanvas);
    }
}

function toCanvasCoords(canvas, pagex, pagey) {
    const boundingRect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
    const worldX = (pagex - boundingRect.left) * scaleX;
    const worldY = (pagey - boundingRect.top) * scaleY;

    return [worldX, worldY];
}


var selectedToken = undefined;
// Click and drag
canvas.addEventListener('mousedown', event => {

    const position = toCanvasCoords(canvas, event.clientX, event.clientY);
    selectedToken = mainscene.find_token(position[0], position[1]);

    // No object clicked.
    if (selectedToken === undefined) {
        return;
    }

    function onDrag(event) {
        const position = toCanvasCoords(canvas, event.clientX, event.clientY);


        selectedToken.set_pos(position[0], position[1]);
        scheduleUpdate();
    }

    // Stop dragging on mouse up.
    function onMouseUp(event) {
        canvas.removeEventListener('mousemove', onDrag);
        canvas.removeEventListener('mouseup', onMouseUp);
        selectedToken = undefined;
        fps_counter.freeze();
    }

    canvas.addEventListener('mouseup', onMouseUp);
    canvas.addEventListener('mousemove', onDrag);

});

// Fit the canvas to the current winow size.
const windowResize = () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    // This clears the canvas, so trigger a re-draw.
    scheduleUpdate();
}
window.addEventListener('resize', windowResize);

windowResize();

