.PHONY: all
all: build

.PHONY: clean
clean:
	rm -rf build

.PHONY: build
build:
	cmake -DCMAKE_BUILD_TYPE=RelWithDebInfo -B build
	cmake --build build -j 8 --config RelWithDebInfo

.PHONY: test
test: build
	cd build && ctest
