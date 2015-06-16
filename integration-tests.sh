#/bin/sh

cargo build

for testdir in testfiles/*; do
	rm -rf testdata
	mkdir testdata
	target/debug/demo < "$testdir/input" > testdata/output
	if diff -u testdata/output "$testdir/output" ; then
		echo "test passed : $testdir"
	else
		echo "test failed : $testdir"
	fi
done

rm -rf testdata
