FILES := $(shell find src/ -name *.rs)

PREFIX := /home/wafelack/.orion/
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
	@cp $(TARGET) $(PREFIX)
	@echo -e "\033[0;32mdone\033[0m"
	@echo -n "* Copying Orion core and standard libraries ... "
	@cp -r $(LIB) $(PREFIX)
	@echo -e "\033[0;32mdone\033[0m"
	@echo "Orion has been installed to $(PREFIX). Please add this location in your PATH variable."

uninstall : $(PREFIX)
	@echo -n "* Removing Orion directory ... "
	@rm -fr $(PREFIX)
	@echo -e "\033[0;32mdone\033[0m"
