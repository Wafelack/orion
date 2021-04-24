FILES := $(shell find src/ -name *.rs)

PREFIX ?= /home/wafelack/.orion
TARGET := target/release/orion
LIB    := lib/

$(TARGET) : $(FILES)
	cargo test
	cargo build --release

install : $(TARGET) $(LIB)
	@echo -n "* Creating Orion directory ... "
	@mkdir -p $(PREFIX)
	@echo -e "\033[0;32mdone\033[0m"
	@echo -n "* Copying Orion binary ... "
	@mkdir -p $(PREFIX)/bin/
	@cp $(TARGET) $(PREFIX)/bin/
	@echo -e "\033[0;32mdone\033[0m"
	@echo -n "* Copying Orion core and standard libraries ... "
	@mkdir -p $(PREFIX)/lib/
	@cp -r $(LIB) $(PREFIX)/lib/orion/
	@echo -e "\033[0;32mdone\033[0m"
	@echo -n "* Adding Orion to PATH ... "
	@echo -e set PATH $(PREFIX)/bin \$$PATH >> /home/wafelack/.config/fish/config.fish
	@echo -e "\033[0;32mdone\033[0m"
	@echo -n "* Setting ORION_LIB ... "
	@echo -e export ORION_LIB=$(PREFIX)/$(LIB)/orion >> /home/wafelack/.config/fish/config.fish
	@echo -e "\033[0;32mdone\033[0m"

uninstall : $(PREFIX)
	@echo -n "* Removing Orion binary ... "
	@rm $(PREFIX)/bin/orion
	@echo -e "\033[0;32mdone\033[0m"
	@echo -n "* Removing Orion library ... "
	@rm -fr $(PREFIX)/lib/orion/
	@echo -e "\033[0;32mdone\033[0m"

