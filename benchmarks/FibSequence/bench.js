const os = require("os");

const fibParam = process.argv[2];
const runCount = process.argv[3];
const libPath = os.platform() == "win32"?
  "target\\debug\\rapl_lib.dll":
  "target/debug/librapl_lib.so"

function fib(n) {
    var a = 0, b = 1, t;
    while (n-- > 0) {
      t = a;
      a = b;
      b += t;
    }
    return a;
  }

const koffi = require('koffi');
const lib = koffi.load(libPath);

const start = lib.func('int start_rapl()');
const stop = lib.func('void stop_rapl()');


for (let i = 0; i < runCount; i++){
    start();

    fib(fibParam);

    stop();
}

console.log("job done");  
