{
    for i in $(find ./src -name "*.rs"); do
    echo "这是$i: "
    cat $i
    echo "\n--------------\n"
    done
} > target/t.txt
