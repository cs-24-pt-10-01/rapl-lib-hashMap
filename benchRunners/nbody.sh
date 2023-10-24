Body_Count=50000000
count=1 #Testing only #TODO: change to actually useful number

echo "!!! Starting N-Body !!!"

#   C
echo --- Starting C ---
gcc -fomit-frame-pointer -march=ivybridge benchmarks/N-Body/C/Also_better_than_rust.c -O3 -o benchmarks/N-Body/C/Also_better_than_rust -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/N-Body/C/Also_better_than_rust $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CNBody"
echo --- C Done ---
echo

#   C++
echo --- Starting C++ ---
g++ -fomit-frame-pointer -march=ivybridge -std=c++17 benchmarks/N-Body/Cpp/better_than_rust.cpp -O3 -o benchmarks/N-Body/Cpp/better_than_rust -L./target/release -lrapl_lib -Wl,-rpath=./target/release && ./benchmarks/N-Body/Cpp/better_than_rust $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CppNBody"
echo --- C++ Done ---
echo

#   Node
echo --- Starting Node.js ---
node ./benchmarks/N-Body/JavaScript/bench.js $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "NodeNBody"
echo --- Node.js Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/N-Body/Python/bench.py $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "PythonNBody"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/N-Body/Python/bench.py $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "PypyNBody"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/N-Body/CSharp/N-Body.csproj --configuration Release $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "CsharpNBody"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/N-Body/Java/Bench.java $Body_Count $count
sleep 5s
bash utils/append_to_latest_csv.sh "JavaNBody"
echo --- Java Done ---
echo

echo "!!! Finished N-Body !!!"

