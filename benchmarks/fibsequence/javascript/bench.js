const os = require("os");

// getting arguments
const fibParam = process.argv[2];
const runCount = process.argv[3];

// finding path depending on OS
const libPath = os.platform() == "win32"?
  "target\\release\\rapl_lib.dll":
  "target/release/librapl_lib.so"

// test method
function fib(n) {
    var a = 0, b = 1, t;
    while (n-- > 0) {
      t = a;
      a = b;
      b += t;
    }
    return a;
  }

// loading library
const koffi = require('koffi');
const lib = koffi.load(libPath);

// loading functions
const start = lib.func('int start_rapl()');
const stop = lib.func('void stop_rapl()');

// running benchmark
for (let i = 0; i < runCount; i++){
    start();

    let result = fib(fibParam);

    stop();
    console.log(result);
}
