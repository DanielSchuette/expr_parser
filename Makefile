# Just a wrapper around cargo to simulate some errors.
BUILD_DIR := /target/debug
BIN := expr_parser
ARGS := -g -f
TREE_FILE := tree.gv
EXPRESSION := -d -e '(18-6)/50*61+4^2'
ERR_EXPRESSION1 := -e '2123^sdkfj(141+22-(5998)-142'
ERR_EXPRESSION2 := -e '2123^(141+22-(5998)-142sdkfj'
ERR_EXPRESSION3 := -e '223^(11+2429-(542)-11'

.PHONY: all test clean help vm err1 err2 err3

$(BUILD_DIR)/$(BIN):
	cargo build

test: $(BUILD_DIR)/$(BIN)
	.$(BUILD_DIR)/$(BIN) $(ARGS) $(TREE_FILE) $(EXPRESSION)

help: $(BUILD_DIR)/$(BIN)
	.$(BUILD_DIR)/$(BIN) --help

vm: $(BUILD_DIR)/$(BIN)
	.$(BUILD_DIR)/$(BIN)

clean:
	rm -f *.gv *.pdf $(BUILD_DIR)/$(BIN)

err1: $(BUILD_DIR)/$(BIN)
	.$(BUILD_DIR)/$(BIN) $(ERR_EXPRESSION1)

err2: $(BUILD_DIR)/$(BIN)
	.$(BUILD_DIR)/$(BIN) $(ERR_EXPRESSION2)

err3: $(BUILD_DIR)/$(BIN)
	.$(BUILD_DIR)/$(BIN) $(ERR_EXPRESSION3)
