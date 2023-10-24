fibInput=200000
count=1000
testName="fib"
folder="FibSequence"

echo "starting fib"

#   Node
node ./benchmarks/$folder/bench.js $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"

#   Python
python3 ./benchmarks/$folder/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"

#   Pypy
pypy ./benchmarks/$folder/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"

#   C#
dotnet run --project ./benchmarks/$folder/benchC#/Fib.csproj --configuration Release $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"

#   Java
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/fibjava/Bench.java $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"
