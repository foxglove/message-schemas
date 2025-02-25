.PHONY: all
all: sdk examples

.PHONY: clean
clean:
	rm -rf examples/ws-server-Release
	rm -rf foxglove-Release

.PHONY: sdk
sdk:
	cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo -S foxglove -B foxglove-Release
	cmake --build foxglove-Release --verbose

.PHONY: examples
examples: sdk
	cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo -S examples/ws-server -B examples/ws-server-Release
	cmake --build examples/ws-server-Release --verbose
