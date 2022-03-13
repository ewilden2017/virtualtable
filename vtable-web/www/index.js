import * as wasm from "vtable-web";
import { memory } from "vtable-web/vtable_web_bg";
import { fps } from "./fps.js";

const TOKEN_STYLES = ["#AA0000", "#00AA00"];
const TOKEN_RADIUS = 20;

// Initialize canvas object.
const canvas = document.getElementById('maincanvas');
const ctx = canvas.getContext('2d');

const fps_counter = new fps();
const fps_display = document.getElementById("fps");

var mainscene;

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
function onMouseDown(event) {

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

}

function startup() {

    // Click and drag
    canvas.addEventListener('mousedown', onMouseDown);

    // Fit the canvas to the current winow size.
    const windowResize = () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        // This clears the canvas, so trigger a re-draw.
        scheduleUpdate();
    }
    window.addEventListener('resize', windowResize);
    windowResize();
}

// Create a websocket to the server.
let scheme = "ws"
if (document.location.protocol === "https:") {
    scheme += "s";
}
let socket = new WebSocket(scheme + "://" + document.location.hostname + "/api/session");

socket.onopen = function (e) {
    console.log("Opened socket.");
    canvas.disabled = false;

    startup();
}

socket.onclose = function (e) {
    console.log("Closed socket.");

    // Remove canvas interaction, other than mouseup.
    canvas.removeEventListener("mousedown", onMouseDown);
    canvas.removeEventListener('mousemove', onDrag);
    clear();
}

socket.onmessage = function (e) {
    console.log("From server: " + e.data);
    wasm.handle_message(e.data);
}


