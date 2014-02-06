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
echo "`time ./tmp/run-go`"

echo "\n##run-rust..."
echo "`time ./tmp/run-rust`"

# clean
echo "\n#clean..."
rm -rf tmp
