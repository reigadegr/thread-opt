#!//bin/sh
echo "rs代码行数: "
find $(dirname "$0")/src ./*.rs -name "*.rs" -exec cat {} \; | wc -l
