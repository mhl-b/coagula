const cvs = document.getElementById("cvs");
const ctx = cvs.getContext('2d');
const offcvs = new OffscreenCanvas(cvs.width, cvs.height);
const offctx = offcvs.getContext('2d');

document.body.oncontextmenu = () => {
    return false
};

WebAssembly.instantiateStreaming(fetch("bin.wasm"), {})
    .then(
        (obj) => {
            const exports = obj.instance.exports;
            const mem = exports.memory;

            console.log(exports);

            const ptr = exports.ptr();
            console.log("ptr:", ptr);
            exports.init(ptr);
            console.log("init");

            //get_hex_bench(exports);

            const hexes = exports.hex_buf(ptr);
            const centers = exports.centers_buf(ptr);
            const corners = exports.corners_buf(ptr);

            const HEX_SIZE = 2;
            const POINT_SIZE = 2;

            console.log("hex", hexes);
            console.log("centers", centers);
            console.log("corners", corners);

            const hex_buf = new Int32Array(mem.buffer, hexes[0], hexes[1] * HEX_SIZE);
            const centers_buf = new Float32Array(mem.buffer, centers[0], centers[1] * POINT_SIZE);
            const corners_buf = new Float32Array(mem.buffer, corners[0], corners[1] * POINT_SIZE);
            console.log("hex buf", hex_buf);
            console.log("centers buf", centers_buf);
            console.log("corners buf", corners_buf);

            const cbs = corners_buf.length;
            offcvs.width = 800;
            offcvs.height = 600;
            console.log(offcvs);
            draw_corners(offctx, corners_buf);
            draw_hex(offctx, centers_buf, hex_buf);

            cvs.addEventListener("wheel", (event) => {
                exports.zoom(ptr, event.deltaY, event.offsetX, event.offsetY);
            }, {passive: true})

            const center_x = cvs.width / 2;
            const center_y = cvs.height / 2;
            cvs.addEventListener("mousedown", (e) => {
                e.preventDefault();
                let x = e.offsetX;
                let y = e.offsetY;
                if (e.button == 0) {
                    let [q, r] = exports.get_hex(ptr, x, y);
                    console.log(`hex=${q}:${r}`);
                } else {
                    const [sw, sh] = exports.get_origin(ptr);
                    exports.adj_origin(ptr, x - center_x, y - center_y);
                    const [sw1, sh1] = exports.get_origin(ptr);
                    console.log(`adj origin x:${sw}->${sw1}, y:${sh}->${sh1}`);
                }
            })

            function loop() {
                ctx.clearRect(0, 0, cvs.width, cvs.height);
                let [sw, sh] = exports.get_origin(ptr);
                let scale = exports.get_scale(ptr);
                ctx.drawImage(offcvs, sw, sh, cvs.width / scale, cvs.height / scale, 0, 0, cvs.width, cvs.height);
                window.requestAnimationFrame(loop)
            }

            loop()
        }
    );

function draw_hex(ctx, cnt, hex) {
    ctx.font = '8px serif'
    const size = cnt.length / 2;
    for (let i = 0; i < size; i++) {
        const x = cnt[i * 2];
        const y = cnt[i * 2 + 1];
        const q = hex[i * 2];
        const r = hex[i * 2 + 1];
        ctx.strokeText(`${q}:${r}`, x - 2, y + 2);
    }
}

function draw_corners(ctx, buf) {
    ctx.beginPath();
    for (let i = 0; i < buf.length; i += 12) {
        let mx = buf[i];
        let my = buf[i + 1];
        ctx.moveTo(mx, my);
        for (j = 0; j < 6; j++) {
            let dx = buf[i + j * 2];
            let dy = buf[i + j * 2 + 1];
            ctx.lineTo(dx, dy);
        }
        ctx.lineTo(mx, my);
    }
    ctx.stroke()
}

