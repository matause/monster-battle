ifeq (WINDOWS,1)
	EXE_SUFFIX=.exe
	LIB_PREFIX=
	LIB_SUFFIX=.dll
else
	EXE_SUFFIX=
	LIB_PREFIX=lib
	LIB_SUFFIX=.so
endif

CORE_DEBUG=target/debug/libmonster_battle_core.rlib
RT_DEBUG=target/debug/monsterbattle${EXE_SUFFIX}
RT_STAGE=stage/monsterbattle${EXE_SUFFIX}
G1_DEBUG=target/debug/${LIB_PREFIX}g1${LIB_SUFFIX}
G1_STAGE=stage/modules/${LIB_PREFIX}g1${LIB_SUFFIX}
G1_DATA_DIR=g1/data
G1_DATA_FILES=$(shell find ${G1_DATA_DIR} -type f)
G1_DATA_STAGE_DIR=stage/modules/g1
G1_DATA_STAGE_FILES=$(patsubst ${G1_DATA_DIR}/%,${G1_DATA_STAGE_DIR}/%,${G1_DATA_FILES})

all: ${RT_DEBUG} ${G1_DEBUG}

${CORE_DEBUG}: $(shell find core -type f)
	cd core && cargo build

${RT_DEBUG}: ${CORE_DEBUG} $(shell find rt -type f)
	cd rt && cargo build --features portable

${G1_DEBUG}: ${CORE_DEBUG} $(shell find g1 -type f)
	cd g1 && cargo build

${RT_STAGE}: ${RT_DEBUG}
	@mkdir -p $(dir $@)
	cp -fT $< $@

${G1_STAGE}: ${G1_DEBUG}
	@mkdir -p $(dir $@)
	cp -fT $< $@

${G1_DATA_STAGE_DIR}/%: ${G1_DATA_DIR}/%
	@mkdir -p $(dir $@)
	cp -fT $< $@

stage: ${RT_STAGE} ${G1_STAGE} ${G1_DATA_STAGE_FILES}

run: stage
	RUST_LOG=debug ./${RT_STAGE}
