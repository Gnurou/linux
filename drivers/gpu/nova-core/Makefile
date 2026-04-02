# SPDX-License-Identifier: GPL-2.0

obj-$(CONFIG_NOVA_CORE) += nova-core.o

nova-core-y := nova_core.o nova_core_exports.o
rustflags-y += --emit=metadata

# Export Rust symbols so dependent modules can use them at runtime.
#
# This is a workaround until the build system supports Rust cross-module
# dependencies natively.
rust_exports = $(NM) -p --defined-only $(1) | awk '$$2=="T" && $$3!~/__(pfx|cfi|odr_asan)/ && $$3!~/:/ && $$3!~/^(init|cleanup)_module$$/ { printf $(2),$$3 }'

define filechk_nova_core_exports
	$(call rust_exports,$(obj)/nova_core.o,"EXPORT_SYMBOL_RUST_GPL(%s);\n")
endef

$(obj)/exports_nova_core_generated.h: $(obj)/nova_core.o FORCE
	$(call filechk,nova_core_exports)

$(obj)/nova_core_exports.o: $(obj)/exports_nova_core_generated.h
