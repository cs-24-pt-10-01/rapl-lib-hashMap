append_to_latest_csv () {
    #finding latest csv file
    FILE=$(ls -t | grep csv | head -1)
    # append string to name
    timestamp=$(date +%s)
    echo $FILE
    mv $FILE "results/${FILE%.csv}_$1_$timestamp.csv"
    
}

# stopping services
bash kill_and_burn.sh 0

echo "starting"

mkdir results
# -- fib --

fibInput=20000
count=1000

#   Node
echo "starting fib"

node ./benchmarks/FibSequence/bench.js $fibInput $count
sleep 5s
append_to_latest_csv "NodeFib"


#   Pypy
pypy ./benchmarks/FibSequence/bench.py $fibInput $count
sleep 5s
append_to_latest_csv "PypyFib"

#   C#
# building
dotnet build ./benchmarks/FibSequence/benchC#  --configuration Release

# running
./benchmarks/FibSequence/benchC#/bin/Debug/net7.0/Fib $fibInput $count
sleep 5s
append_to_latest_csv "CsharpFib" 

# starting services
bash kill_and_burn.sh 1
