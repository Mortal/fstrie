/* From https://youtu.be/zmtHaZG7pPc?t=22m19s */
typedef void fstrie_db_t;

struct fstrie_error {
    char *message;
    int failed;
    int code;
};

void fstrie_init();

fstrie_db_t *fstrie_load(const char *root, struct fstrie_error *);
void fstrie_unload(fstrie_db_t *, struct fstrie_error *);
char **fstrie_lookup(fstrie_db_t *, const char *key, struct fstrie_error *);

void fstrie_free(char *);
void fstrie_free_list(char **);
