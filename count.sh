#!//bin/sh
echo "rs代码行数: "
find ${0%/*}/src -name "*.rs" -exec cat {} \; | wc -l

echo "shadow的py代码行数: "
find ${0%/*}/*.py ${0%/*}/make*/*.py  -name "*.py" -exec cat {} \; | wc -l
