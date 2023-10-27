import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

import java.math.BigInteger;

class Bench {
    public static void main(String[] args) {

        // Finding os
        var os = System.getProperty("os.name");

        // Finding the path of library (and loading it)
        var dll_path = System.getProperty("user.dir") + "/target/release/";
        if (os.equals("Linux")) {
            dll_path = dll_path + "librapl_lib.so";
        } else if (os.equals("Windows 11")) {
            dll_path = dll_path + "rapl_lib.dll";
        } else {
            System.out.println("OS not supported");
            return;
        }

        System.load(dll_path);

        // Loading functions
        MemorySegment start_rapl_symbol = SymbolLookup.loaderLookup().find("start_rapl").get();
        MethodHandle start_rapl = Linker.nativeLinker().downcallHandle(start_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));

        MemorySegment stop_rapl_symbol = SymbolLookup.loaderLookup().find("stop_rapl").get();
        MethodHandle stop_rapl = Linker.nativeLinker().downcallHandle(stop_rapl_symbol,
                    FunctionDescriptor.of(ValueLayout.JAVA_INT));

        
        // Getting arguments
        int n = Integer.parseInt(args[0]);
        int loop_count = Integer.parseInt(args[1]);

        // Running benchmark
        // Note that this could potentially be optimized away
        // by the compiler due to the fact that the result is not used.
        for (int i = 0; i < loop_count; i++) {
            try {
                start_rapl.invoke();
            } catch (Throwable e) {
                e.printStackTrace();
            }

            BigInteger result = itFibBig(n);

            try {
                stop_rapl.invoke();
            } catch (Throwable e) {
                e.printStackTrace();
            }
            System.out.println(result.toString());
        }
    }

    // Test method
    public static long itFibN(int n)
    {
        if (n < 2)
        return n;
        long ans = 0;
        long n1 = 0;
        long n2 = 1;
        for(n--; n > 0; n--)
        {
            ans = n1 + n2;
            n1 = n2;
            n2 = ans;
        }
        return ans;
    }

    // Modified version of the itFibN method that uses BigInteger
    public static BigInteger itFibBig(int n)
    {
        if (n < 2)
        return new BigInteger(n + "");
        BigInteger ans = new BigInteger("0");
        BigInteger n1 = new BigInteger("0");
        BigInteger n2 = new BigInteger("1");
        for(n--; n > 0; n--)
        {
            ans = n1.add(n2);
            n1 = n2;
            n2 = ans;
        }
        return ans;
    }
}