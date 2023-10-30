testName="n-body"
folder="n-body"
count=1 #Testing only #TODO: change to actually useful number
body_count=50000000

echo "!!! Starting $testName !!!"
echo

#   C
echo --- Starting C ---
gcc -fomit-frame-pointer -march=ivybridge benchmarks/$folder/c/bench.c -O3 -o benchmarks/$folder/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/$folder/c/bench $count $body_count
sleep 5s
bash utils/append_to_latest_csv.sh "C$testName"
echo --- C Done ---
echo

#   C++
echo --- Starting C++ ---
g++ -fomit-frame-pointer -march=ivybridge -std=c++17 benchmarks/$folder/cpp/bench.cpp -O3 -o benchmarks/$folder/cpp/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/$folder/cpp/bench $count $body_count
sleep 5s
bash utils/append_to_latest_csv.sh "Cpp$testName"
echo --- C++ Done ---
echo

#   Node
echo --- Starting JavaScript ---
node ./benchmarks/$folder/javascript/bench.js $count $body_count
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"
echo --- JavaScript Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/$folder/python/bench.py $count $body_count
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/$folder/python/bench.py $count $body_count
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/$folder/csharp/Bench.csproj --configuration Release $count $body_count
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/java/Bench.java $count $body_count
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"
echo --- Java Done ---
echo

echo "!!! Finished $testName !!!"

