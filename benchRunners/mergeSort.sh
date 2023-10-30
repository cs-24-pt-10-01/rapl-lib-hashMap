testName="mergeSort"
folder="mergesort"
count=1000
mergeInput=`cat benchRunners/mergeSortParam` # getting input from file

echo "!!! Starting $testName !!!"
echo

#   Node
echo --- Starting JavaScript ---
node ./benchmarks/$folder/javascript/bench.js $count $mergeInput 
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"
echo --- JavaScript Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/$folder/python/bench.py $count $mergeInput
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/$folder/python/bench.py $count $mergeInput
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/$folder/csharp/Bench.csproj --configuration Release $count $mergeInput
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/java/Bench.java $count $mergeInput
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"
echo --- Java Done ---
echo

#   C
echo --- Starting C ---
gcc benchmarks/MergeSort/c/bench.c -O3 -o benchmarks/MergeSort/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/MergeSort/c/bench $count $mergeInput
sleep 5s
bash utils/append_to_latest_csv.sh "C$testName"
echo --- C Done ---
echo

#   C++
echo --- Starting C++ ---
g++ benchmarks/MergeSort/cpp/bench.cpp -O3 -o benchmarks/MergeSort/cpp/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/MergeSort/cpp/bench $count $mergeInput
sleep 5s
bash utils/append_to_latest_csv.sh "Cpp$testName"
echo --- C++ Done ---
echo

echo "!!! Finished $testName !!!"
