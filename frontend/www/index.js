import {GameManager} from "tetris20g-ai-frontend";
import {greet} from "tetris20g-ai-frontend";

async function getWeights() {
  const response = await fetch("static/weights__1.txt");
  return response.text();
}

function render(field, current_piece) {
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
      pre.textContent = render(m.render_field(), m.render_current_piece());
      start = timestamp;
    }

    requestAnimationFrame(renderLoop);
  };
  requestAnimationFrame(renderLoop);
});
