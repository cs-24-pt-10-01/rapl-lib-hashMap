Body_Count=50000000
count=1 #Testing only #TODO: change to actually useful number

echo "!!! Starting N-Body !!!"
echo

#   C
echo --- Starting C ---
gcc -fomit-frame-pointer -march=ivybridge benchmarks/n-body/c/bench.c -O3 -o benchmarks/n-body/c/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/n-body/c/bench $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CNBody"
echo --- C Done ---
echo

#   C++
echo --- Starting C++ ---
g++ -fomit-frame-pointer -march=ivybridge -std=c++17 benchmarks/n-body/cpp/bench.cpp -O3 -o benchmarks/n-body/cpp/bench -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/n-body/cpp/bench $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CppNBody"
echo --- C++ Done ---
echo

#   Node
echo --- Starting JavaScript ---
node ./benchmarks/n-body/javascript/bench.js $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "NodeNBody"
echo --- JavaScript Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/n-body/python/bench.py $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "PythonNBody"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/n-body/python/bench.py $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "PypyNBody"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/n-body/csharp/N-Body.csproj --configuration Release $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CsharpNBody"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/n-body/java/Bench.java $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "JavaNBody"
echo --- Java Done ---
echo

echo "!!! Finished N-Body !!!"

