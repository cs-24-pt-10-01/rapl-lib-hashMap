fibInput=20000
count=1000

echo "starting fib"

#   Node
node ./benchmarks/FibSequence/bench.js $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "NodeFib"

#   Python
pypy ./benchmarks/FibSequence/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "PythonFib"

#   Pypy
pypy ./benchmarks/FibSequence/bench.py $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "PypyFib"

#   C#
dotnet run --project ./benchmarks/FibSequence/benchC#/Fib.csproj --configuration Release $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "CsharpFib"

#   Java
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/FibSequence/fibjava/Bench.java $fibInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "JavaFib"
