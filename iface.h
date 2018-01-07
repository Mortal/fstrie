/* From https://youtu.be/zmtHaZG7pPc?t=22m19s */
typedef void lsm_view_t;
typedef struct lsm_error_s {
	char *message;
	int failed;
	int code;
} lsm_error_t;

char *lsm_view_dump_memdb(const lsm_view_t *view,
			  unsigned int *len_out,
			  int with_source_contents,
			  int with_names,
			  lsm_error_t *err);
