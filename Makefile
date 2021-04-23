FILES=src/errors.rs src/interpreter.rs src/lexer.rs src/main.rs src/parser.rs src/tests.rs
BINARY=target/release/orion
LIB=lib/

$(BINARY) : $(FILES)
	cargo build --release
install : $(BINARY)
	cp $(BINARY) /usr/bin/
	mkdir -p /usr/lib/orion/
	cp -r $(LIB)/* /usr/lib/orion/
	@printf "Orion and its library have been installed to the default location (/usr/lib/orion/).\nIf you want to use another location, move the /usr/lib/orion folder to a new location and set the ORION_LIB variable to the new path."

uninstall :
	@printf "\033[1;33mWARNING\033[0m: This will uninstall orion at its default location. If you used another location, you will have to delete it manually.\n"
	rm /usr/bin/orion -rf /usr/lib/orion/
