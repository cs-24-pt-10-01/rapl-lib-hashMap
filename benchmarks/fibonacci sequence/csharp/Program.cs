using System;
using System.Runtime.InteropServices;

const int count = 100;
const int fibVal = 47;
const string pathToLib = @"target\debug\rapl_lib.dll";

// DLL imports
[DllImport(pathToLib)]
static extern int start_rapl();

[DllImport(pathToLib)]
static extern void stop_rapl();

// test method
static ulong Fib(uint x)
{
    if (x == 0) return 0;

    ulong prev = 0;
    ulong next = 1;
    for (int i = 1; i < x; i++)
    {
        ulong sum = prev + next;
        prev = next;
        next = sum;
    }
    return next;
}

//testing
for (int i = 0; i < count; i++)
{
    start_rapl();

    Fib(fibVal);

    stop_rapl();
}
Console.WriteLine("job done");
