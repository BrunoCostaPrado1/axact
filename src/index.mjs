import { h, render } from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

const html = htm.bind(h);

function App(props) {
  return html` <div>
    ${props.cpus.map((cpu) => {
      return html`
        <div class="bar">
          <div class="bar-inner" style="width: ${cpu}%">
            <label> ${cpu.toFixed(2)} </label>
          </div>
        </div>
      `;
    })}
  </div>`;
}

let i = 0;

// let update = async () => {
//   let response = await fetch("/api/cpus");
//   if (response.status !== 200) {
//     throw new Error(`HTTP Error!status:${response.status}}`);
//   }
//   let json = await response.json();
//   render(html`<${App} cpus=${json}></${App}>`, document.body);
// };

// update();
// setInterval(update, 1000);

let url = new URL("/realtime/cpus", window.location.href);
url.protocol = url.protocol.replace("http", "ws");

let ws = new WebSocket(url.href);
ws.onmessage = (ev) => {
  let json = JSON.parse(ev.data);
  render(html`<${App} cpus=${json}></${App}>`, document.body);
};
