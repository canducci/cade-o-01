// Carrega o game.wasm real e confere a ponte (ponteiro, tamanho, UTF-8).
import { readFileSync } from 'node:fs';
const { instance } = await WebAssembly.instantiate(readFileSync(new URL('game.wasm', import.meta.url)));
const w = instance.exports;
const read = () => new TextDecoder('utf-8', { fatal: true }).decode(new Uint8Array(w.memory.buffer, w.out_ptr(), w.out_len()));
w.start(42, 0);
if (!read().includes('PARÓDIA')) throw new Error('tela inicial');
w.act(0); w.act(0);
const s = read();
if (!/⏳ \d+h/.test(s) || !s.includes('> Viajar')) throw new Error(s);
w.act(0);
// interrogar custa 2h: horas = prazo - 2
const m = read().match(/^@city \d+ \d+ \d+ \d+ 1 1 (\d+) (\d+)\n/);
if (!m || +m[1] !== m[2] - 2) throw new Error('linha de cena: ' + read().split('\n')[0]);
// a fala da testemunha vem numa linha própria, que o JS põe no balão
if (!/\n" .+: "/.test(read())) throw new Error('fala: ' + read());
console.log(read());
// carreira: o total entra no início e volta na linha de cena de fim
w.start(42, 7);
w.act(0); w.act(0);
while (!/^@(win|lose) /.test(read())) w.act(0);
if (!/^@lose \d+ 7 0\n/.test(read())) throw new Error('cena de fim: ' + read().split('\n')[0]);
console.log('ok');
