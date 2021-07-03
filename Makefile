# Shell options
RM	?= rm
FIND	?= find

# Git options
GIT	?= git

# Rust options
CARGO	?= cargo

# Path variables
SRC_DIR	  := $(CURDIR)/src

.DEFAULT_GOAL := build

.PHONY: clean
clean:
	$(FIND) $(SRC_DIR) -type f -name '*.proto.*' -exec rm -f {} \;
	
.PHONY: build
build: clean
	$(CARGO) build --color=always

.PHONY: publish
publish:
	$(CARGO) publish --no-verify --allow-dirty
	
.PHONY: update
update:
	$(GIT) submodule foreach git pull