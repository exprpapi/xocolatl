import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import "xterm/css/xterm.css";
import './index.css';

// TODO(webpack/webpack#6615): use `import { riscv } from './pkg';` instead
import('../pkg').then(main).catch(console.error);

function main(wasm) {
  wasm.set_panic_hook();

  const term = ((term) => {
    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(document.querySelector('#monitor'));
    fitAddon.fit();
    return term;
  })(new Terminal());

  function put(args) {
    term.write(args);
  }

  function print(args) {
    put(args);
    put('\r\n');
  }

  const WELCOME = `\
                                              ▄▄                  ▄▄  \r
      ▀███▀   ▀██▀                          ▀███           ██   ▀███  \r
        ███▄  ▄█                              ██           ██     ██  \r
         ▀██▄█▀     ▄██▀██▄ ▄██▀██  ▄██▀██▄   ██  ▄█▀██▄ ██████   ██  \r
           ███     ██▀   ▀███▀  ██ ██▀   ▀██  ██ ██   ██   ██     ██  \r
         ▄█▀▀██▄   ██     ███      ██     ██  ██  ▄█████   ██     ██  \r
        ▄█   ▀██▄  ██▄   ▄███▄    ▄██▄   ▄██  ██ ██   ██   ██     ██  \r
      ▄██▄▄  ▄▄███▄ ▀█████▀ █████▀  ▀█████▀ ▄████▄████▀██▄ ▀████▄████▄\r

Welcome to xocolatl. Type \`help\` to see what's possible right now.`;

  const CLEAR_SCREEN = '\x1b[3J';

  function repl() {
    const PROMPT = '$ ';
    while (true) {
      put(PROMPT);
      break;
    }
  }

  print(WELCOME);
  repl();

  const instruction = 0x07b00513;
  const disassembly = wasm.disassemble(instruction);
  console.log(`wasm.disassemble(${instruction}) = ${disassembly}`);
}
