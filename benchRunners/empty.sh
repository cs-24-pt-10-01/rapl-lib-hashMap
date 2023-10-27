count=1000
testName="empty"
folder="empty"

echo "!!! Starting $testName !!!"
echo

#   C
echo --- Starting C ---
gcc benchmarks/empty/c/bench.c -O3 -o benchmarks/empty/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/empty/c/bench $count
sleep 5s
bash utils/append_to_latest_csv.sh "CEmpty"
echo --- C Done ---
echo

#   C++
echo --- Starting C++ ---
g++ benchmarks/empty/cpp/bench.cpp -O3 -o benchmarks/empty/cpp/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/empty/cpp/bench $count
sleep 5s
bash utils/append_to_latest_csv.sh "CppEmpty"
echo --- C++ Done ---
echo

#   Node
echo --- Starting JavaScript ---
node ./benchmarks/empty/javascript/bench.js $count
sleep 5s
bash utils/append_to_latest_csv.sh "NodeEmpty"
echo --- JavaScript Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/empty/python/bench.py $count
sleep 5s
bash utils/append_to_latest_csv.sh "PythonEmpty"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/empty/python/bench.py $count
sleep 5s
bash utils/append_to_latest_csv.sh "PypyEmpty"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/empty/csharp/Empty.csproj --configuration Release $count
sleep 5s
bash utils/append_to_latest_csv.sh "CsharpEmpty"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/empty/java/Bench.java $count
sleep 5s
bash utils/append_to_latest_csv.sh "JavaEmpty"
echo --- Java Done ---
echo

echo "!!! Finished $testName !!!"
