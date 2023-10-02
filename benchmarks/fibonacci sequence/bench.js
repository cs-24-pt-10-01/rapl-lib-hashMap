const runCount = 100
const fibParam = 47

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
const lib = koffi.load('target\\debug\\rapl_lib.dll');

const start = lib.func('int start_rapl()');
const stop = lib.func('void stop_rapl()');


for (let i = 0; i < runCount; i++){
    start();

    fib(fibParam);

    stop();
}

console.log("job done");  