# Makefile based on https://github.com/getsentry/symbolic
PYTHON=python3

all: wheel

wheel:
	$(PYTHON) setup.py bdist_wheel

IMAGE=quay.io/pypa/manylinux1_x86_64

wheel-manylinux:
	d=`mktemp --tmpdir -d manylinux.XXXXXX` && ( \
	  git archive HEAD | tar -x -C $$d && \
	docker run --rm -it -v $$d:/work -w /work $(IMAGE) sh manylinux.sh && \
	cp $$d/dist/*.whl -t dist ) ; \
	$(RM) -r $$d

.PHONY: all wheel wheel-manylinux
