// bindgen-flags: --result-error-enum "MyResult"
// bindgen-parse-callbacks: result-error-enum-rename

enum MyResult {
    MyResultOk = 0,
    MyResultErr1,
    MyResultErr2,
};

enum MyResult do_something(void);
