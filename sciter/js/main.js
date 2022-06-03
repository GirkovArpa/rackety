import { $, $$ } from '@sciter';
import { launch } from '@env';
import { spawn } from '@sys';

main();

async function main() {
  adjustWindow();
}

function adjustWindow() {
 const [_, w] = document.state.contentWidths();
 const h = document.state.contentHeight(w);
 const [sw, sh] = Window.this.screenBox('frame', 'dimension');
 Window.this.move((sw - w) / 2, (sh - h) / 2, w, h, true);
}

Window.this.on('statechange', () => {
  if (Window.this.state === Window.WINDOW_MINIMIZED) {
    Window.this.state = Window.WINDOW_HIDDEN;
    spawn(['ahk/taskbar.exe']);
    showTrayIcon();
    console.log('minimized');
  } else if (Window.this.state === Window.WINDOW_SHOWN) {
    Window.this.trayIcon('remove');
    console.log('shown');
    Window.this.activate(true);
  }
});

document.on('click', 'a', (_, a) => {
  launch(a.attributes.href);
  return true;
});

$('#menu-volume').on('click', (evt, el) => {
  console.log(el.value);
});

$('#menu-volume-slider').on('click', (evt, el) => {
  console.log(el.value);
});

$('#volume-slider').on('input', (_, { value }) => {
  $('#menu-volume-slider').value = value;
  $('#volume-label').textContent = `VOLUME: ${value}%`;
  Window.this.xcall('set_volume', value);
  Window.this.xcall('click');
});

$('#menu-volume-slider').on('input', (_, { value }) => {
  $('#volume-slider').value = value;
  $('#volume-label').textContent = `VOLUME: ${value}%`;
  Window.this.xcall('set_volume', value);
  Window.this.xcall('click');
});

document.on('keydown', 'textarea', (evt, el) => {
  //AUDIO.play();
});

globalThis.PAUSED = false;

document.on('click', '#pause, #menu-pause', (evt, el) => {
  PAUSED = !PAUSED;
  $('#menu-pause').textContent = PAUSED ? 'Resume' : 'Pause';
  $('#pause').textContent = PAUSED ? 'Resume' : 'Pause';
  Window.this.xcall('pause', PAUSED);
});

$('#menu-about').on('click', () => {
  Window.this.modal({ url: 'this://app/html/about.html' });
});

$('#about').on('click', () => {
  Window.this.modal({ url: 'this://app/html/about.html' });
});

$('#menu-send-feedback').on('click', () => {
  launch('https://girkovarpa.itch.io/rackety?ref=rackety');
});

$('#quit').on('click', () => Window.this.close());

$('#menu-quit').on('click', () => Window.this.close());

$('#menu-show-window').on('click', () => {
  Window.this.state = Window.WINDOW_SHOWN;
});

Window.this.on('trayiconclick', ({ data }) => {
  const [sx, sy] = Window.this.box('position', 'client', 'screen');
  const menu = document.$('menu#tray');
  const { screenX, screenY } = data;
  menu.popupAt(screenX - sx, screenY - sy, 2);
});

async function showTrayIcon() {
  Window.this.trayIcon({
    image: await Graphics.Image.load('this://app/png/32x32.png'),
    text: 'Rackety: Sound FX for typists!',
  });
}

$('#menu-start-at-login > button').on('click', (evt, el) => {
  $('#start-at-login').checked = el.checked;
  Window.this.xcall('start_at_login', el.checked);
  console.log(el.checked);
});

$('#start-at-login').on('click', (evt, el) => {
  $('#menu-start-at-login > button').checked = el.checked;
  Window.this.xcall('start_at_login', el.checked);
  console.log(el.checked);
});
