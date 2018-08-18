SOURCES := $(sort $(wildcard */*.rs))
TARGETS := $(patsubst %.rs,%.run,$(SOURCES))
all: $(TARGETS)

%.run: %.rs
	@echo $< "=>" $@
	@./run.sh $<
	@echo

clean:
	rm -f */*.run

push: clean all
	git push
