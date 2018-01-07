import sys
import argparse
from fstrie import Database


parser = argparse.ArgumentParser()
parser.add_argument('root')


def main():
    args, rest = parser.parse_known_args()
    with Database(args.root) as db:
        for line in rest or sys.stdin:
            print('\n'.join(db[line.strip()]))


if __name__ == '__main__':
    main()
