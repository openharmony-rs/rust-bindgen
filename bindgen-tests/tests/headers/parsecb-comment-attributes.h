// bindgen-flags: --no-layout-tests
// bindgen-parse-callbacks: comment-attributes

/** A variable. */
extern int variable;

/** A function. */
void function(void);

/** A type alias. */
typedef int Alias;

/** A struct. */
struct Struct {
    /** A field. */
    int field;
};

/** An enum. */
enum Enum {
    /** A variant. */
    Variant,
};

/** A forward declared struct alias. */
typedef struct Forward Forward;
