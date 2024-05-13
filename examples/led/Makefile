CXBE=$(NXDK_DIR)/tools/cxbe/cxbe

CRATE=led

all:
	$(MAKE) -f $(NXDK_DIR)/Makefile NXDK_ONLY=y
	cargo +nightly build --release
	mkdir -p bin
	@$(CXBE) target/i386-xbox-win32/release/$(CRATE).exe -OUT:bin/default.xbe -TITLE:$(CRATE)
	mkdir -p output
	xdvdfs pack bin "output/$(CRATE).iso"

clean:
	cargo clean
	rm -rf bin
	rm -rf output
	rm -f bin.iso

.PHONY: clean
