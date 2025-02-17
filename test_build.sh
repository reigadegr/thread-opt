cargo fmt
rm -rf output
rm -rf $(find ./target -name "*thread-opt*")
rm -rf $(find ./target -name "*thread_opt*")
python3 ./make.py build --debug --nightly
