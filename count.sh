#!//bin/sh
echo "rs代码行数: "
find $(dirname "$0")/src ./*.rs -name "*.rs" -exec cat {} \; | wc -l

echo "shadow的py代码行数: "
find $(dirname "$0")/*.py $(dirname "$0")/make*/*.py  -name "*.py" -exec cat {} \; | wc -l
