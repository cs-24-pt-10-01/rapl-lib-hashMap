fibInput=200000
count=1000
testName="fib"
folder="fibsequence"

echo "!!! Starting $testName !!!"
echo

#   Node
echo --- Starting JavaScript ---
node ./benchmarks/$folder/javascript/bench.js $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"
echo --- JavaScript Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/$folder/python/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/$folder/python/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/$folder/csharp/Fib.csproj --configuration Release $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/java/Bench.java $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"
echo --- Java Done ---
echo

echo "!!! Finished $testName !!!"
