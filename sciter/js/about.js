import { $, $$ } from '@sciter';
import { launch } from '@env';

$('#sciter').on('click', () => {
  launch('https://sciter.com/?ref=rackety');
});

$('#terra-informatica').on('click', () => {
  launch('https://terrainformatica.com/?ref=rackety');
});

$('#girkov-arpa').on('click', () => {
  launch('https://girkovarpa.itch.io/?ref=rackety');
});

$('button').on('click', () => Window.this.close());
