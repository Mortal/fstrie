import sys
import codecs
import argparse
from fstrie import Database


parser = argparse.ArgumentParser()
parser.add_argument('root')


def main():
    args = parser.parse_args()
    with Database(args.root) as db:
        for line in sys.stdin:
            print(codecs.encode(repr(db[line.strip()]), 'rot13'))


if __name__ == '__main__':
    main()
