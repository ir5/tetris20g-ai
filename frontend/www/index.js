import {GameManager} from "tetris20g-ai-frontend";
import {greet} from "tetris20g-ai-frontend";

async function getWeights() {
  const response = await fetch("static/weights__1.txt");
  return response.text();
}

const pp = 22;

const canvas = document.getElementById("canvas")
canvas.height = pp * 26;
canvas.width = pp * 12;

const empty = '.'.charCodeAt();
let colormap1 = [];
colormap1['I'.charCodeAt()] = "#cc0000";
colormap1['O'.charCodeAt()] = "#cccc00";
colormap1['S'.charCodeAt()] = "#cc00cc";
colormap1['Z'.charCodeAt()] = "#00cc00";
colormap1['L'.charCodeAt()] = "#cc6600";
colormap1['J'.charCodeAt()] = "#0000cc";
colormap1['T'.charCodeAt()] = "#00cccc";

let colormap2 = [];
colormap2['I'.charCodeAt()] = "#ff0000";
colormap2['O'.charCodeAt()] = "#ffff00";
colormap2['S'.charCodeAt()] = "#ff00ff";
colormap2['Z'.charCodeAt()] = "#00ff00";
colormap2['L'.charCodeAt()] = "#ff8800";
colormap2['J'.charCodeAt()] = "#0000ff";
colormap2['T'.charCodeAt()] = "#00ffff";

function render(field, current_piece) {
  let ctx = canvas.getContext('2d');

  // draw grid
  const offx = pp;
  const offy = pp * 5;
  ctx.beginPath();
  // ctx.strokeStyle = "#111111";


  for (let i = 0; i < 20; i++) {
    for (let j = 0; j < 10; j++) {
      const idx = i * 10 + j;
      let color = null;
      if (field[idx] != empty) {
        color = colormap1[field[idx]];
      } else if (current_piece[idx] != empty) {
        color = colormap2[current_piece[idx]];
      } else {
        color = "#ffffff";
      }

      if (color) {
        ctx.beginPath();
        ctx.fillStyle = color;
        ctx.fillRect(
          offx + pp * j + 1, offy + pp * i + 1,
          pp, pp
        );
      }
    }
  }

  for (let j = 0; j <= 10; j++) {
    ctx.moveTo(0.5 + offx + pp * j, offy);
    ctx.lineTo(0.5 + offx + pp * j, offy + pp * 20);
  }
  for (let i = 0; i <= 20; i++) {
    ctx.moveTo(offx, 0.5 + offy + pp * i);
    ctx.lineTo(offx + pp * 10, 0.5 + offy + pp * i);
  }
  ctx.lineWidth = 1;
  ctx.stroke();
}

function render_debug(field, current_piece) {
  console.log();
  let a = "";
  for (let i = 0; i < 20; i++) {
    for (let j = 0; j < 10; j++) {
      let c = "";
      if (field[i * 10 + j] != '.'.charCodeAt()) {
        c = String.fromCharCode(field[i * 10 + j]);
      } else if (current_piece[i * 10 + j] != '.'.charCodeAt()) {
        c = String.fromCharCode(current_piece[i * 10 + j]);
      } else {
        c = ".";
      }
      a += c;
    }
    a += "\n"
  }
  return a;
}

const keys = "IOSZJLT";
let seq = "";
for (let i = 0; i < 10000; i++) {
  seq += keys[Math.floor(Math.random() * 7)];
}
console.log(seq);

getWeights().then(data => {
  let m = GameManager.new(data, seq);
  let pre = document.getElementById("canvas");

  let start;
  const renderLoop = (timestamp) => {
    if (start === undefined) {
      start = timestamp;
    }
    const elapsed = timestamp - start;

    if (elapsed > 60) {
      m.act();
      render(m.render_field(), m.render_current_piece());
      start = timestamp;
    }

    requestAnimationFrame(renderLoop);
  };
  requestAnimationFrame(renderLoop);
});
