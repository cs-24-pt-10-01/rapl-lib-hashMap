mergeInput=`cat benchRunners/mergeSortParam` # getting input from file
count=1000
testName="mergeSort"
folder="mergesort"

echo "!!! Starting $testName !!!"
echo

#   Node
echo --- Starting JavaScript ---
node ./benchmarks/$folder/javascript/bench.js $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Node$testName"
echo --- JavaScript Done ---
echo

#   Python
echo --- Starting Python ---
python3 ./benchmarks/$folder/python/bench.py $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Python$testName"
echo --- Python Done ---
echo

#   Pypy
echo --- Starting PyPy ---
pypy ./benchmarks/$folder/python/bench.py $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Pypy$testName"
echo --- PyPy Done ---
echo

#   C#
echo --- Starting C# ---
dotnet run --project ./benchmarks/$folder/csharp/MergeSort.csproj --configuration Release $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Csharp$testName"
echo --- C# Done ---
echo

#   Java
echo --- Starting Java ---
java --enable-native-access=ALL-UNNAMED --enable-preview --source 21 ./benchmarks/$folder/java/Bench.java $mergeInput $count
sleep 5s
bash utils/append_to_latest_csv.sh "Java$testName"
echo --- Java Done ---
echo

echo "!!! Finished $testName !!!"
