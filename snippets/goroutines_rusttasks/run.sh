# prepare
mkdir tmp

# test
echo "#test"
echo "##test go..."
go test -v ./...

echo "\n##test rust..."
rustc --test -o ./tmp/test-rust rust.rs
./tmp/test-rust

# build
echo "\n#build..."
go build -o ./tmp/run-go go.go
rustc -o ./tmp/run-rust rust.rs

# run
echo "#run..."
echo "\n##run-go..."

date >> result.txt
echo "run-go" >> result.txt
/usr/bin/time ./tmp/run-go >> result.txt 2>&1

echo "\n##run-rust..."
echo "run-rust" >> result.txt
/usr/bin/time ./tmp/run-rust >> result.txt 2>&1 

# clean
echo "\n#clean..."
rm -rf tmp
