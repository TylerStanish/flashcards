import argparse
from datetime import datetime
import json
from os import path
from pydoc import pager
import sys


cards = {}


def save_cards():
    with open('cards.json', 'w') as f:
        f.write(json.dumps(cards, indent=2, sort_keys=True, default=str))


def load_cards():
    global cards
    if not path.exists('cards.json'):
        with open('cards.json', 'w+') as f:
            f.write('{}')
    with open('cards.json', 'r') as f:
        cards = json.loads(f.read())


def format_cards():
    lines = []
    for key, val in cards.items():
        lines.append(f'{val["en"]} - {val["sp"]} - {val["last_practiced"]}')
    return '\n'.join(lines)


def main():
    parser = argparse.ArgumentParser(prog='flashcards')
    parser.add_argument('mode', choices=['ls', 'create', 'practice'])
    # add flags for last practiced, least
    #practiced, weakest words, etc priority?
    args = parser.parse_args()
    load_cards()
    if args.mode == 'ls':
        # list all cards
        pager(format_cards())
    elif args.mode == 'create':
        # create a card
        en = input('Enter the english word:\n')
        sp = input('Enter the spanish word:\n')
        cards[en] = {
            'en': en,
            'sp': sp,
            'last_practiced': datetime.utcnow(),
        }
        save_cards()
    elif args.mode == 'practice':
        # practice a card
        print('practicing')
    else:
        # if done correctly, the argument parser should never allow this code to
        # reach here
        raise ValueError('Invalid mode, select from "ls", "create", or "practice"')


if __name__ == '__main__':
    main()

