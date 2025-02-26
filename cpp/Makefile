.PHONY: all
all: sdk-cpp examples

.PHONY: clean
clean:
	rm -rf examples/ws-server-Release
	rm -rf foxglove-Release

.PHONY: sdk-c
sdk-c:
	cd ../c/foxglove && cargo build --release

.PHONY: sdk-cpp
sdk-cpp: sdk-c
	cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo -S foxglove -B foxglove-Release
	cmake --build foxglove-Release -j 8

.PHONY: examples
examples: sdk-cpp
	cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo -S examples/ws-server -B examples/ws-server-Release
	cmake --build examples/ws-server-Release -j 8

.PHONY: test
test: sdk-cpp
	cd foxglove-Release && ctest
