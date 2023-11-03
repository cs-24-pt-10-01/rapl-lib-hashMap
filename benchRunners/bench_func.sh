runbenchmark(){
    language=$1
    testName=$2
    cmd=$3
    input=$4
    inputSize=$5
    echo --- Starting $language ---
    $cmd $input
    sleep 5s
    bash utils/append_to_latest_csv.sh "$language$testName$inputSize"
    echo --- $language Done ---
    echo
}

#Structure of runbenchmark call (remember to include this file at the top of bechmark sh file): 
#runbenchmark language testName cmd (input) (inputSize)