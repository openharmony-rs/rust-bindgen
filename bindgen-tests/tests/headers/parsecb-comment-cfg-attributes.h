// bindgen-flags: --no-layout-tests --newtype-enum NewtypeEnum --bitfield-enum BitfieldEnum --newtype-global-enum GlobalEnum --constified-enum-module ModuleEnum --constified-enum ConstsEnum
// bindgen-parse-callbacks: comment-cfg-attributes

/** A struct with bitfield accessors, cfg-guarded. */
struct Struct {
    /** A field. */
    unsigned int a : 1;
    unsigned int b : 2;
};

/** A newtype enum, cfg-guarded. */
enum NewtypeEnum {
    /** A variant, cfg-guarded. */
    NewtypeA,
    /** A variant without attributes. */
    NewtypeB,
};

/** A bitfield enum, cfg-guarded. */
enum BitfieldEnum {
    BitfieldA = 1,
    BitfieldB = 2,
};

/** A global newtype enum, cfg-guarded. */
enum GlobalEnum {
    GlobalA,
    GlobalB,
};

/** A module enum, cfg-guarded. */
enum ModuleEnum {
    ModuleA,
    ModuleB,
};

/** A consts enum, cfg-guarded. */
enum ConstsEnum {
    ConstsA,
    ConstsB,
};

/** An anonymous enum, cfg-guarded. */
enum {
    AnonA,
    AnonB,
};

/** A variable, cfg-guarded. */
extern int variable;

/** A function, cfg-guarded. */
void function(void);
