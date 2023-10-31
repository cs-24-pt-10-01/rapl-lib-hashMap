// general benchmark imports
import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
// benchmark specific imports
import java.util.*;
import java.util.stream.Stream;
import java.util.stream.Collectors;

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
        // converting json array to java array
        String[] data = args[1].replace("[","").replace("]","").split(",");
        List<Long> sortParam = Arrays.stream(data).map(String::trim).map(Long::valueOf).toList();
        int loop_count = Integer.parseInt(args[0]);

        // Running benchmark
        for (int i = 0; i < loop_count; i++) {
            try {
                start_rapl.invoke();
            } catch (Throwable e) {
                e.printStackTrace();
            }

            List<Long> sorted = quickSort(sortParam);

            try {
                stop_rapl.invoke();
            } catch (Throwable e) {
                e.printStackTrace();
            }

            // stopping compiler optimizations
            if (sorted.size() < 42) {
                System.out.println(sorted);
            }
        }

    }

    // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
    // Rosetta code

    public static <E extends Comparable<? super E>> List<E> quickSort(List<E> arr) {
        if (arr.isEmpty())
            return arr;
        else {
            E pivot = arr.get(0);
    
            List<E> less = new LinkedList<E>();
            List<E> pivotList = new LinkedList<E>();
            List<E> more = new LinkedList<E>();
    
            // Partition
            for (E i: arr) {
                if (i.compareTo(pivot) < 0)
                    less.add(i);
                else if (i.compareTo(pivot) > 0)
                    more.add(i);
                else
                    pivotList.add(i);
            }
    
            // Recursively sort sublists
            less = quickSort(less);
            more = quickSort(more);
    
            // Concatenate results
            less.addAll(pivotList);
            less.addAll(more);
            return less;
        }
    }
}