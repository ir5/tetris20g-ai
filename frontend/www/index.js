import {GameManager} from "tetris20g-ai-frontend";
import {greet} from "tetris20g-ai-frontend";

async function getWeights() {
  const response = await fetch("static/weights__1.txt");
  return response.text();
}

const keys = "IOSZJLT";
let seq = "";
for (let i = 0; i < 10000; i++) {
  seq += keys[Math.floor(Math.random() * 7)];
}
console.log(seq);

getWeights().then(data => {
  let m = GameManager.new(data, seq);
  m.act();
  console.log(m.render_field());
  console.log(m.render_current_piece());
  //console.log(m.field());
  // let pre = document.getElementById("canvas");
  // pre.textContent
});
