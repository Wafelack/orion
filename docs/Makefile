OUTPUT = messy-orion-reference.pdf
INPUT = title.tex
MAIN = main.tex

$(OUTPUT) : $(INPUT) $(MAIN)
	latexmk -pdf $(MAIN)
	@mv $(patsubst %.tex, %.pdf, $(MAIN)) $(OUTPUT)
	@latexmk -c

.PHONY : clean
clean :
	@latexmk -C

see : $(OUTPUT)
	@firefox-bin ./$(OUTPUT)
