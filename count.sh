#!//bin/sh
echo "代码行数: "
find ${0%/*}/src -name "*.rs" -exec cat {} \; | wc -l
