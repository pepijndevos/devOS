PREFIX = arm-none-eabi-
CC = $(PREFIX)gcc
AS = $(PREFIX)as
AR = $(PREFIX)ar
LD = $(PREFIX)ld
OBJCOPY = $(PREFIX)objcopy
OBJDUMP = $(PREFIX)objdump
CARGO = xargo
LDFLAGS = --gc-sections -nostdlib -T link.ld
RUSTLIB = target/arm-none-eabi/debug/libdevos.a

.PHONY : all clean $(RUSTLIB)

$(RUSTLIB):
	$(CARGO) build --target=arm-none-eabi

devos.o: boot.o $(RUSTLIB)
	$(LD) $(LDFLAGS) -o $@ $^

devos: devos.o
	$(OBJCOPY) -O binary $< $@

uImage: devos
	mkimage -C none -A arm -T kernel -O linux -a 0xC0008000 -e 0xC0008000 -d $< $@

all: devos

clean:
	rm -rf *.o devos target
