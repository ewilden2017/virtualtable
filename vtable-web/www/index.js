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

// Interface for client state.
let state = wasm.State.new();

// Drawing Functions
//
function clear() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
}

function drawToken(token, x, y) {
    console.log("Printing token, token:" + token + ", x: " + x + ", y: " + y)
    ctx.beginPath();
    // TODO store radius in token data?
    ctx.arc(x, y, token.radius, 0, 2 * Math.PI, false);
    ctx.fillStyle = TOKEN_STYLES[token.style];
    ctx.fill();
    ctx.lineWidth = 1;
    ctx.strokeStyle = "#000000";
    ctx.stroke();
}

var updatePending = false;
function updateCanvas() {
    console.log("Updating canvas");
    const ms_frame = fps_counter.update();
    const avg_fps = fps_counter.get_fps();
    fps_display.textContent = `FPS: ${Math.round(avg_fps)} (${ms_frame} ms)`;

    updatePending = false;
    clear();

    state.each_token(drawToken);
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

function onDrag(event) {
    const position = toCanvasCoords(canvas, event.clientX, event.clientY);


    state.move_token(selectedToken, position[0], position[1]);
    scheduleUpdate();
}

// Stop dragging on mouse up.
function onMouseUp(event) {
    canvas.removeEventListener('mousemove', onDrag);
    canvas.removeEventListener('mouseup', onMouseUp);
    selectedToken = undefined;
    fps_counter.freeze();
}

function onMouseDown(event) {

    const position = toCanvasCoords(canvas, event.clientX, event.clientY);
    selectedToken = state.token_at_pos(position[0], position[1]);

    // No object clicked.
    if (selectedToken === undefined) {
        return;
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

function socketSend(msg) {
    socket.send(msg);
}

socket.onopen = function (e) {
    state.set_send(socketSend);
    // Set up the current scene right away for now.
    const scene_data = {
        msg: {
            type: "Fetch",
            data_type: "Scene",
            id: 0,
        },
    };
    socket.send(JSON.stringify(scene_data));
    console.log("Opened socket.");
    canvas.disabled = false;

    //startup();
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
    state.handle_message(e.data);

    // Once we process the scene message, start doing things.
    if (state.check_ready()) {
        startup();
    }
}


